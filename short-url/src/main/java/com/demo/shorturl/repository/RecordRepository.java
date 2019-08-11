package com.demo.shorturl.repository;

import com.demo.shorturl.model.Record;
import org.springframework.data.mongodb.repository.ReactiveMongoRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface RecordRepository extends ReactiveMongoRepository<Record, String> {

}
