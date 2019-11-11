package metadata;

import exception.CustomValidatorException;

import java.util.Set;

/**
 * customized validator
 */
public interface CustomValidator {

    /**
     * @param tableNames sql tableNames(dbName.tableName)
     * @throws CustomValidatorException Throw the exception if the validation fails
     */
    void valid(Set<String> tableNames) throws CustomValidatorException;

}
