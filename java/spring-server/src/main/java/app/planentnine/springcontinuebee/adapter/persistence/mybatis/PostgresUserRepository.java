package app.planentnine.springcontinuebee.adapter.persistence.mybatis;

import app.planentnine.springcontinuebee.adapter.persistence.entity.PostgresUserEntity;
import app.planentnine.springcontinuebee.adapter.util.UuidTypeHandler;
import org.apache.ibatis.annotations.Insert;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Result;
import org.apache.ibatis.annotations.Results;
import org.apache.ibatis.annotations.Select;

import java.util.Optional;
import java.util.UUID;

public interface PostgresUserRepository {
    @Select("SELECT * FROM account WHERE user_uuid = #{userUuid}")
    @Results(id = "userResultMap", value = {
            @Result(property = "id", column = "id", javaType = UUID.class, typeHandler = UuidTypeHandler.class),
            @Result(property = "userUuid", column = "user_uuid", javaType = UUID.class, typeHandler = UuidTypeHandler.class),
            @Result(property = "publicKey", column = "public_key", javaType = String.class),
            @Result(property = "hash", column = "hash", javaType = String.class)
    })
    Optional<PostgresUserEntity> loadUserByUuid(@Param("userUuid") UUID userUuid);
    
    @Insert("INSERT INTO account (id, user_uuid, public_key, hash) " +
            "VALUES (#{id}, #{userUuid}, #{publicKey}, #{hash})")
    void createNewUser(PostgresUserEntity postgresUserEntity);
}
