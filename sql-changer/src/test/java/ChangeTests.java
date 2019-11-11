import executer.SqlExecutor;
import org.apache.calcite.sql.parser.SqlParseException;
import org.apache.calcite.tools.RelConversionException;
import org.apache.calcite.tools.ValidationException;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.time.LocalDateTime;

public class ChangeTests {


    public static void main(String[] args) throws Exception {
        String resultSql;
        LocalDateTime start = LocalDateTime.now();
//        resultSql = changeMysql();
        resultSql = changePresto();
//        resultSql = changeHive();
//        resultSql = changeDruid();
//        resultSql = changeEs();
//        resultSql = changeMongo();
        LocalDateTime end = LocalDateTime.now();
        System.out.println("ResultSql: " + resultSql);
    }


    private static String changeMysql() throws SqlParseException, ValidationException, RelConversionException, IOException {
        String sql = "select table1.name,10+15 from test.demo as table1 left join hello.world as table2 on table1.name = table2.name where table1.name = 'sam'";
        return SqlExecutor.builder()
                .init(sql, SqlResultType.MYSQL)
//                .customValidator(new TestValidator())
                .ok()
                .change();
    }

    private static String changePresto() throws SqlParseException, ValidationException, RelConversionException, IOException {
        String sql = "select * from demo.test where 'status' = 'FAILED' limit 10";
        return SqlExecutor.builder()
                .init(sql, SqlResultType.PRESTO)
                .ok()
                .change();
    }

    private static String changeHive() throws SqlParseException, ValidationException, RelConversionException, IOException {
        String sql = "select * from demo.test where 'status' = 'FAILED' limit 10";
        return SqlExecutor.builder()
                .init(sql, SqlResultType.HIVE)
                .ok()
                .change();
    }

    private static String changeDruid() throws SqlParseException, ValidationException, RelConversionException, IOException {
        String sql = "select * from druid.test where age = 16 limit 10";
        return SqlExecutor.builder()
                .init(sql, SqlResultType.DRUID)
                .ok()
                .change();
    }

    private static String changeEs() throws SqlParseException, ValidationException, RelConversionException, IOException {
        String sql = "select _MAP['name'] from elasticsearch.`demo` group by _MAP['name']";
        return SqlExecutor.builder()
                .init(sql, SqlResultType.ES)
                .ok()
                .change();
    }

    private static String changeMongo() throws SqlParseException, ValidationException, RelConversionException, IOException {
        String sql = "select * from demo.task where 'status' = 'FAILED' limit 10";
        return SqlExecutor.builder()
                .init(sql, SqlResultType.MONGO)
                .ok()
                .change();
    }
}
