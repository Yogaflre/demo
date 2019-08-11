package com.demo.shorturl.model;

import lombok.Data;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;

@Data
@Document
public class UrlMapping {

    @Id
    private String id;
    private String originalUrl;
    private String shortPath;

}
