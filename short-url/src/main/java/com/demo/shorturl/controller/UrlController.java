package com.demo.shorturl.controller;

import com.demo.shorturl.service.UrlService;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;

import javax.annotation.Resource;


@RestController
@RequestMapping("url")
public class UrlController {

    @Resource
    private UrlService urlService;

    /**
     * get one short url
     *
     * @param originalUrl http://localhost:8080/shortUrl/demo/original/demo
     * @return short url
     */
    @GetMapping("one")
    public Mono<String> getShortUrl(@RequestParam String originalUrl) {
        return urlService.getShortUrl(originalUrl);
    }

}
