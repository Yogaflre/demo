package com.yogafire;

import org.junit.Test;

public class AopTest {

    @Test
    public void testTime() throws Exception {
        Aop aop = new Aop();
        aop.checkTime("com.yogafire.Base", "process");
    }

}
