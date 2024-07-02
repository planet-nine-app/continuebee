package app.planentnine.springcontinuebee.adapter.persistence.mybatis;

import app.planentnine.springcontinuebee.adapter.persistence.entity.PostgresUserEntity;
import app.planentnine.springcontinuebee.adapter.util.UuidTypeHandler;
import org.apache.ibatis.annotations.Delete;
import org.apache.ibatis.annotations.Insert;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Result;
import org.apache.ibatis.annotations.Results;
import org.apache.ibatis.annotations.Select;
import org.apache.ibatis.annotations.Update;

import java.util.Optional;
import java.util.UUID;

public interface PostgresUserRepository {
    @Select("SELECT * FROM account WHERE user_uuid = #{userUUID}")
    @Results(id = "userResultMap", value = {
            @Result(property = "id", column = "id", javaType = UUID.class, typeHandler = UuidTypeHandler.class),
            @Result(property = "userUUID", column = "user_uuid", javaType = UUID.class, typeHandler = UuidTypeHandler.class),
            @Result(property = "publicKey", column = "public_key", javaType = String.class),
            @Result(property = "hash", column = "hash", javaType = String.class)
    })
    Optional<PostgresUserEntity> loadUserByUuid(@Param("userUUID") UUID userUUID);
    
    @Insert(
            "INSERT INTO account (id, user_uuid, public_key, hash) " +
            "VALUES (#{id}, #{userUUID}, #{publicKey}, #{hash}) ")
    void createNewUser(PostgresUserEntity postgresUserEntity);
    
    @Update("UPDATE account SET hash = #{newHash} " +
            "WHERE user_uuid = #{userUUID}")
    long updateHash(UUID userUUID, String newHash);
    
    @Delete("DELETE FROM account WHERE user_uuid = #{userUUID}")
    void deleteUserByUuid(UUID userUUID);
}
