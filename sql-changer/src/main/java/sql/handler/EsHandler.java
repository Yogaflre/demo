package sql.handler;

import calcite.elasticsearch.ElasticsearchRel;
import calcite.elasticsearch.ElasticsearchSchemaFactory;
import com.google.common.collect.ImmutableList;
import exception.ParseException;
import metadata.ResourceReader;
import org.apache.calcite.adapter.enumerable.EnumerableConvention;
import org.apache.calcite.adapter.enumerable.EnumerableRel;
import org.apache.calcite.adapter.enumerable.EnumerableRules;
import org.apache.calcite.plan.RelOptPlanner;
import org.apache.calcite.plan.RelTraitSet;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.schema.Schema;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.calcite.tools.Program;
import org.apache.calcite.tools.Programs;
import org.apache.calcite.tools.RuleSet;
import org.apache.calcite.tools.RuleSets;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class EsHandler implements SqlHandler {
    @Override
    public SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> indexes) throws IOException {
        Map<String, Object> esConfig = ResourceReader.getJdbcResources(SqlResultType.ES);
        if (!indexes.isEmpty()) {
            for (String index : indexes) {
                Map<String, Object> configMap = new HashMap<>(esConfig);
                String[] split = index.split("\\.", 2);
                //扫描ES数据库
                configMap.put("index", split[1]);
                Schema schema = new ElasticsearchSchemaFactory().create(null, null, configMap);
                rootSchema.add(split[0], schema);
            }
        } else {
            throw new ParseException("es index is null!");
        }
        return rootSchema;
    }

    @Override
    public String createSql(RelNode relNode) {
        final RuleSet rules = RuleSets.ofList(
                EnumerableRules.ENUMERABLE_PROJECT_RULE,
                EnumerableRules.ENUMERABLE_FILTER_RULE,
                EnumerableRules.ENUMERABLE_AGGREGATE_RULE,
                EnumerableRules.ENUMERABLE_SORT_RULE,
                EnumerableRules.ENUMERABLE_TABLE_SCAN_RULE
        );
        Program program = Programs.of(rules);
        RelOptPlanner planner = relNode.getCluster().getPlanner();
        RelTraitSet traits = planner.emptyTraitSet().replace(EnumerableConvention.INSTANCE);
        EnumerableRel esNode = (EnumerableRel) program.run(planner, relNode, traits, ImmutableList.of(), ImmutableList.of());

        ElasticsearchRel.Implementor implementor = new ElasticsearchRel.Implementor();
        try {
            return implementor.toEsJson(esNode);
        } catch (Exception e) {
            throw new RuntimeException("es change error:" + e.getMessage());
        }
    }
}
