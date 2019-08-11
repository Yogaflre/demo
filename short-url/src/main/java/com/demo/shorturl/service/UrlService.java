package com.demo.shorturl.service;

import com.demo.shorturl.model.UrlMapping;
import com.demo.shorturl.repository.UrlRepository;
import com.demo.shorturl.utils.ConvertUtils;
import lombok.extern.slf4j.Slf4j;
import org.springframework.data.redis.core.ReactiveStringRedisTemplate;
import org.springframework.http.server.reactive.ServerHttpRequest;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;

import javax.annotation.Resource;

/**
 * redis max id : 3521614606207
 */
@Slf4j
@Service
public class UrlService {

    @Resource
    private UrlRepository urlRepository;
    @Resource
    private StatisticService statisticService;
    @Resource
    private ReactiveStringRedisTemplate reactiveStringRedisTemplate;

    private static String DOMAIN = "http://localhost:8080/short/";
    private static String ID_KEY = "short-url:id";


    public Mono<String> getShortUrl(String originalUrl) {
        return reactiveStringRedisTemplate.opsForValue().increment(ID_KEY)
                .doOnSuccess(this::resetUrlId)
                .map(ConvertUtils::base62)
                .doOnSuccess(path -> {
                    UrlMapping mapping = new UrlMapping();
                    mapping.setOriginalUrl(originalUrl);
                    mapping.setShortPath(path);
                    urlRepository.save(mapping).subscribe();
                })
                .map(path -> DOMAIN + path)
                .doOnError(error -> log.error("[url][error] get short url error", error));
    }
    
    public Mono<String> getOriginalUrl(ServerHttpRequest request) {
        return urlRepository.findByShortPath(request.getURI().getPath().split("/")[2])
                .switchIfEmpty(Mono.error(new NullPointerException(request.getURI().getPath() + "")))
                .map(UrlMapping::getOriginalUrl)
                .doOnSuccess(original -> statisticService.statistic(request, original))
                .doOnError(error -> log.error("[url][error] get original url error", error))
                .onErrorReturn("http://localhost:8080/shortUrl/error");
    }


    /**
     * redis id reset 0
     *
     * @param id short-url:id
     */
    private void resetUrlId(Long id) {
        Mono.just(id)
                .flatMap(num -> {
                    if (num >= 2L) {
                        return reactiveStringRedisTemplate.opsForValue().delete(ID_KEY);
                    } else {
                        return Mono.empty();
                    }
                })
                .doOnError(error -> log.error("[url][error] reset redis id error", error))
                .subscribeOn(Schedulers.parallel())
                .subscribe();
    }

}
