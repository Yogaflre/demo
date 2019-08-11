package com.demo.shorturl.repository;

import com.demo.shorturl.model.UrlMapping;
import org.springframework.data.mongodb.repository.ReactiveMongoRepository;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Mono;

@Repository
public interface UrlRepository extends ReactiveMongoRepository<UrlMapping, String> {

    Mono<UrlMapping> findByShortPath(String shortPath);

}
