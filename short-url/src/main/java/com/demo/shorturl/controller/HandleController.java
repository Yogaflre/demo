package com.demo.shorturl.controller;

import com.demo.shorturl.service.UrlService;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.server.reactive.ServerHttpRequest;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.reactive.result.view.Rendering;
import reactor.core.publisher.Mono;

import javax.annotation.Resource;

/**
 * handle request
 */
@Slf4j
@Controller
public class HandleController {

    @Resource
    private UrlService urlService;

    /**
     * redirect to original url
     *
     * @param request server request
     * @return rendering
     */
    @GetMapping("short/{shortUrl}")
    public Mono<Rendering> handle(ServerHttpRequest request) {
        return Mono.from(urlService.getOriginalUrl(request))
                .map(originalUrl -> Rendering.redirectTo(originalUrl).build());
    }

    /**
     * original url test demo
     *
     * @return demo
     */
    @GetMapping("shortUrl/demo/original/demo")
    @ResponseBody
    public Mono<String> test() {
        return Mono.just("Hello, this is a short url demo.");
    }

    /**
     * default error page
     *
     * @return demo
     */
    @GetMapping("shortUrl/error")
    @ResponseBody
    public Mono<String> testError() {
        return Mono.just("Sorry, you get error short url.");
    }
}
