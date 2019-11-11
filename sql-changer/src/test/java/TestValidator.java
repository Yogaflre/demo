import exception.CustomValidatorException;
import metadata.CustomValidator;

import java.util.Set;

/**
 * myself test validator
 */
public class TestValidator implements CustomValidator {
    @Override
    public void valid(Set<String> tableNames) throws CustomValidatorException {
        tableNames.forEach(tableName -> {
            if (tableName.contains("test.")) {
                throw new CustomValidatorException("Test Database is no permission:" + tableName);
            }
        });
    }
}
