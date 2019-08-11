package com.demo.shorturl.service;

import com.demo.shorturl.model.Record;
import com.demo.shorturl.repository.RecordRepository;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.server.reactive.ServerHttpRequest;
import org.springframework.stereotype.Service;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;

import javax.annotation.Resource;
import java.util.Objects;

@Slf4j
@Service
public class StatisticService {

    @Resource
    private RecordRepository recordRepository;

    /**
     * statistic request information
     *
     * @param request request
     */
    public void statistic(ServerHttpRequest request, String originalUrl) {
        Mono.just(request)
                .flatMap(r -> {
                    Record record = new Record();
                    record.setHeaders(r.getHeaders().toSingleValueMap());
                    record.setIp(Objects.requireNonNull(r.getRemoteAddress()).getAddress().getHostAddress());
                    //FIXME 因为转发和生成在一个服务，所以用"/short"来标识
                    record.setShortUrl(r.getURI().getPath().split("/")[2]);
                    record.setOriginalUrl(originalUrl);
                    return recordRepository.save(record);
                })
                .doOnError(error -> log.error("[statistic][error] statistic click error", error))
                .subscribeOn(Schedulers.parallel())
                .subscribe();
    }

}
