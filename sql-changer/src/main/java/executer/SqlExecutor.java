package executer;

import exception.SqlResultTypeException;
import lombok.AllArgsConstructor;
import metadata.CustomValidator;
import metadata.SqlReader;
import org.apache.calcite.avatica.util.Casing;
import org.apache.calcite.avatica.util.Quoting;
import org.apache.calcite.config.CalciteConnectionConfig;
import org.apache.calcite.config.CalciteConnectionConfigImpl;
import org.apache.calcite.config.Lex;
import org.apache.calcite.jdbc.CalciteSchema;
import org.apache.calcite.plan.Context;
import org.apache.calcite.plan.RelOptCluster;
import org.apache.calcite.plan.RelTraitDef;
import org.apache.calcite.plan.hep.HepPlanner;
import org.apache.calcite.plan.hep.HepProgram;
import org.apache.calcite.plan.hep.HepProgramBuilder;
import org.apache.calcite.prepare.CalciteCatalogReader;
import org.apache.calcite.prepare.PlannerImpl;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.RelRoot;
import org.apache.calcite.rel.rules.SubQueryRemoveRule;
import org.apache.calcite.rel.type.RelDataTypeSystem;
import org.apache.calcite.rex.RexBuilder;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.calcite.sql.SqlDialect;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.dialect.HiveSqlDialect;
import org.apache.calcite.sql.dialect.MysqlSqlDialect;
import org.apache.calcite.sql.parser.SqlParseException;
import org.apache.calcite.sql.parser.SqlParser;
import org.apache.calcite.sql.type.SqlTypeFactoryImpl;
import org.apache.calcite.sql.validate.SqlConformance;
import org.apache.calcite.sql.validate.SqlConformanceEnum;
import org.apache.calcite.sql.validate.SqlValidator;
import org.apache.calcite.sql.validate.SqlValidatorUtil;
import org.apache.calcite.sql2rel.RelDecorrelator;
import org.apache.calcite.sql2rel.SqlToRelConverter;
import org.apache.calcite.tools.*;
import sql.enums.SqlResultType;
import sql.handler.*;

import java.io.IOException;
import java.util.List;
import java.util.Properties;
import java.util.Set;

@AllArgsConstructor
public class SqlExecutor {

    private Builder builder;

    public static Builder builder() {
        return new Builder();
    }

    public static class Builder {
        private String sql;
        private SqlResultType sqlResultType;
        private SqlHandler sqlHandler;
        private CustomValidator validator;

        public Builder init(String sql, SqlResultType sqlResultType) {
            this.sql = sql;
            this.sqlResultType = sqlResultType;
            this.sqlHandler = getSqlHandler(sqlResultType);
            return this;
        }

        public Builder customValidator(CustomValidator validator) {
            this.validator = validator;
            return this;
        }

        public SqlExecutor ok() {
            return new SqlExecutor(this);
        }

        private SqlHandler getSqlHandler(SqlResultType sqlResultType) {
            switch (sqlResultType) {
                case HIVE:
                    return new HiveHandler();
                case PRESTO:
                    return new PrestoHandler();
                case DRUID:
                    return new DruidHandler();
                case PHOENIX:
                    return new PhoenixHandler();
                case MYSQL:
                    return new MysqlHandler();
                case ES:
                    return new EsHandler();
                case MONGO:
                    return new MongoHandler();
                default:
                    throw new SqlResultTypeException("Unknown sqlResultType : " + sqlResultType.name());
            }
        }
    }

    public String change() throws IOException, SqlParseException, ValidationException, RelConversionException {
        Set<String> tableNames = SqlReader.getTableNames(builder.sql);
        if (builder.validator != null) {
            builder.validator.valid(tableNames);
        }
        SchemaPlus rootSchema = Frameworks.createRootSchema(true);
        SchemaPlus schemaPlus = builder.sqlHandler.createSchema(rootSchema, tableNames);

        final SqlParser.Config parserConfig = SqlParser.configBuilder()
                .setConformance(SqlConformanceEnum.MYSQL_5)
                .setQuoting(Quoting.BACK_TICK)
                .setCaseSensitive(false)
                .setQuotedCasing(Casing.UNCHANGED)
                .setUnquotedCasing(Casing.UNCHANGED)
                .build();

        final SqlToRelConverter.Config convertConfig = SqlToRelConverter.configBuilder()
                .withTrimUnusedFields(false)
                .withConvertTableAccess(false)
                .withExpand(false)
                .build();

        final HepProgram program = new HepProgramBuilder()
                .addRuleInstance(SubQueryRemoveRule.PROJECT)
                .addRuleInstance(SubQueryRemoveRule.FILTER)
                .addRuleInstance(SubQueryRemoveRule.JOIN)
                .build();

        FrameworkConfig frameworkConfig = Frameworks.newConfigBuilder()
                .parserConfig(parserConfig)
                .defaultSchema(schemaPlus)
                .traitDefs((List<RelTraitDef>) null)
                .sqlToRelConverterConfig(convertConfig)
                .build();

        Planner planner = Frameworks.getPlanner(frameworkConfig);
        SqlNode parse = planner.parse(builder.sql);
        SqlNode validate = planner.validate(parse);
        RelNode relNode = planner.rel(validate).rel;
        HepPlanner hepPlanner = new HepPlanner(program);
        hepPlanner.setRoot(relNode);
        RelNode bestNode = hepPlanner.findBestExp();
        return builder.sqlHandler.createSql(bestNode);
    }

}
