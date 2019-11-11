package com.demo.shorturl.model;

import lombok.Data;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;

import java.util.Map;

/**
 * request record
 */
@Data
@Document
public class Record {
    @Id
    private String id;
    private String shortUrl;
    private String originalUrl;
    private String ip;
    private Map<String, String> headers;

}
