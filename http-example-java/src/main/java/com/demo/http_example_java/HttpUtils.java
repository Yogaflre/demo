package com.demo.http_example_java;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.apache.http.HttpEntity;
import org.apache.http.client.HttpResponseException;
import org.apache.http.client.methods.CloseableHttpResponse;
import org.apache.http.client.methods.HttpGet;
import org.apache.http.client.methods.HttpPost;
import org.apache.http.client.methods.HttpRequestBase;
import org.apache.http.entity.ContentType;
import org.apache.http.entity.StringEntity;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.util.EntityUtils;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.stream.Collectors;

/**
 * Created by yogafire on 2020/10/15
 */
public class HttpUtils {

    private static CloseableHttpClient client = new HttpClientExample().build();

    public static void customClient(CloseableHttpClient client) {
        HttpUtils.client = client;
    }

    public static String get(String url, Map<String, String> params, Map<String, String> headers) throws IOException {

        HttpGet get = buildGet(url, params, headers);
        HttpEntity entity = execute(get);
        return EntityUtils.toString(entity, StandardCharsets.UTF_8);
    }

    public static String postJson(String url, Map<Object, Object> params, Map<String, String> headers) throws IOException {
        StringEntity reqEntity = new StringEntity(new ObjectMapper().writeValueAsString(params), ContentType.APPLICATION_JSON);
        headers.put("Content-Type", ContentType.APPLICATION_JSON.toString());
        HttpPost post = buildPost(url, reqEntity, headers);
        HttpEntity resEntity = execute(post);
        return EntityUtils.toString(resEntity);
    }

    public static String postForm(String url, Map<Object, Object> params, Map<String, String> headers) throws IOException {
        StringEntity reqEntity = new StringEntity(new ObjectMapper().writeValueAsString(params), ContentType.APPLICATION_FORM_URLENCODED);
        headers.put("Content-Type", ContentType.APPLICATION_FORM_URLENCODED.toString());
        HttpPost post = buildPost(url, reqEntity, headers);
        HttpEntity resEntity = execute(post);
        return EntityUtils.toString(resEntity);
    }

    private static HttpGet buildGet(String url, Map<String, String> params, Map<String, String> headers) {
        if (url == null || url.isEmpty()) {
            throw new IllegalArgumentException("url is empty");
        }
        if (params != null && params.size() > 0) {
            url += "?" + params.entrySet().stream()
                    .map(entry -> entry.getKey() + "=" + entry.getValue()).collect(Collectors.joining("&"));
        }
        HttpGet get = new HttpGet(url);
        fillHeader(get, headers);
        return get;
    }


    private static HttpPost buildPost(String url, HttpEntity entity, Map<String, String> headers) {
        if (url == null || url.isEmpty()) {
            throw new IllegalArgumentException("url is empty");
        }
        HttpPost post = new HttpPost(url);
        post.setEntity(entity);
        fillHeader(post, headers);
        return post;
    }

    private static void fillHeader(HttpRequestBase base, Map<String, String> headers) {
        if (headers != null && headers.size() > 0) {
            for (Map.Entry<String, String> entry : headers.entrySet()) {
                base.setHeader(entry.getKey(), entry.getValue());
            }
        }
    }

    private static HttpEntity execute(HttpRequestBase request) throws IOException {
        CloseableHttpResponse response = client.execute(request);
        int statusCode = response.getStatusLine().getStatusCode();
        if (statusCode == 200 || statusCode == 201) {
            return response.getEntity();
        } else {
            throw new HttpResponseException(statusCode, "status code is not 200 or 201");
        }
    }

}
