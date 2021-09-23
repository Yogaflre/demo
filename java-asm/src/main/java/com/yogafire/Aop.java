package com.yogafire;

import javassist.ClassPool;
import javassist.CtClass;
import javassist.CtMethod;

public class Aop {

    public void checkTime(String className, String methodName) throws Exception {
        ClassPool pool = ClassPool.getDefault();
        CtClass ctClass = pool.getCtClass(className);
        CtMethod ctMethod = ctClass.getDeclaredMethod(methodName);
        ctMethod.addLocalVariable("startTime", CtClass.longType);
        ctMethod.insertBefore("startTime = System.currentTimeMillis();");
        ctMethod.insertAfter("System.out.println((System.currentTimeMillis() - startTime) + \"ms\");");
        Class<?> clazz = ctClass.toClass();
        clazz.getDeclaredMethod(methodName).invoke(clazz.newInstance());
    }

}
