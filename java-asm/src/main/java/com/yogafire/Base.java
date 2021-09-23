package com.yogafire;

public class Base {
    private String name;
    public Integer age;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Integer getAge() {
        return age;
    }

    public void setAge(Integer age) {
        this.age = age;
    }

    public void process() throws Exception {
        System.out.println("Base process...");
        Thread.sleep(200);
    }
}
