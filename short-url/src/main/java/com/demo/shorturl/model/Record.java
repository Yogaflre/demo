package com.demo.shorturl.model;

import lombok.Data;

import java.util.Map;

/**
 * request record
 */
@Data
public class Record {

    private String id;
    private String shortUrl;
    private String originalUrl;
    private String ip;
    private Map<String, String> headers;

}
