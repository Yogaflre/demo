package metadata;

import lombok.extern.slf4j.Slf4j;
import org.apache.calcite.avatica.util.Casing;
import org.apache.calcite.avatica.util.Quoting;
import org.apache.calcite.config.Lex;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.parser.SqlParseException;
import org.apache.calcite.sql.parser.SqlParser;
import org.apache.calcite.sql.validate.SqlConformanceEnum;

import java.util.Set;

/**
 * parser utils
 */
@Slf4j
public class SqlReader {

    public static Set<String> getTableNames(String sql) throws SqlParseException {
        final SqlParser.Config parserConfig = SqlParser.configBuilder()
                .setConformance(SqlConformanceEnum.MYSQL_5)
                .setQuoting(Quoting.BACK_TICK)
                .setCaseSensitive(true)
                .setQuotedCasing(Casing.UNCHANGED)
                .setUnquotedCasing(Casing.UNCHANGED)
                .build();
        SqlParser parser = SqlParser.create(sql, parserConfig);
        SqlNode sqlNode = parser.parseStmt();
        return getTableNames(sqlNode);
    }

    public static Set<String> getTableNames(SqlNode sqlNode) {
        return sqlNode.accept(new TableNameReader());
    }

}
