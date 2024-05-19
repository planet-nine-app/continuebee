package app.planentnine.springcontinuebee.adapter.web;

import app.planentnine.springcontinuebee.adapter.web.dto.RestHashDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestHashDtoMapper;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestUserDtoMapper;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.port.incoming.CreateUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.DeleteUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.InsertHashIfNoneUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.VerifyHashUseCase;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestParam;

import java.time.LocalDateTime;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;


@Controller("/user")
public class UserController {
    
    private final CreateUserUseCase createUserUseCase;
    private final DeleteUserUseCase deleteUserUseCase;
    private final InsertHashIfNoneUseCase insertHashIfNoneUseCase;
    private final VerifyHashUseCase verifyHashUseCase;
    private final RestUserDtoMapper userDtoMapper;
    private final RestHashDtoMapper hashDtoMapper;
    
    @Autowired
    public UserController(CreateUserUseCase createUserUseCase,
                          DeleteUserUseCase deleteUserUseCase,
                          InsertHashIfNoneUseCase insertHashIfNoneUseCase,
                          VerifyHashUseCase verifyHashUseCase,
                          RestUserDtoMapper restUserDtoMapper,
                          RestHashDtoMapper restHashDtoMapper) {
        this.createUserUseCase = createUserUseCase;
        this.deleteUserUseCase = deleteUserUseCase;
        this.insertHashIfNoneUseCase = insertHashIfNoneUseCase;
        this.verifyHashUseCase = verifyHashUseCase;
        this.userDtoMapper = restUserDtoMapper;
        this.hashDtoMapper = restHashDtoMapper;
    }
    
    @PutMapping("/{publicKey}")
    public ResponseEntity<Object> createUser(@PathVariable String publicKey,
                                             @RequestParam(required = false) String hash) {
        try {
            RestUserDto userDto = RestUserDto.builder()
                    .userUUID(null)
                    .pubKey(publicKey)
                    .hash(hash)
                    .build();
            
            User user = createUserUseCase.createUser(userDtoMapper.map(userDto));
            Map<String, String> responseMap = new HashMap<>();
            responseMap.put("userUuid", user.userUuid().toString());
            return ResponseEntity.accepted().body(responseMap);
        } catch (ValidationException e) {
            return ResponseEntity.badRequest().body(e.getErrors());
        }
    }
    
    @GetMapping("/{uuid}")
    public ResponseEntity<Object> verifyHash(@PathVariable UUID uuid,
                                             @RequestParam LocalDateTime timestamp,
                                             @RequestParam String hash,
                                             @RequestParam String[] signature) {
        try {
            RestHashDto hashDto = RestHashDto.builder()
                    .uuid(uuid)
                    .timestamp(timestamp)
                    .hash(hash)
                    .signature(signature)
                    .build();
            
            boolean verified = verifyHashUseCase.verifyHash(hashDtoMapper.map(hashDto));
            if (verified) {
                return ResponseEntity.accepted().body(uuid.toString());
            } else {
                return ResponseEntity.badRequest().body("Bad Request");
            }
        } catch (ValidationException e) {
            return ResponseEntity.badRequest().body(e.getErrors());
        }
    }
    
    @PostMapping("/save-hash")
    public ResponseEntity<Object> saveHash(@RequestBody User user) {
        if (insertHashIfNoneUseCase.insertHashIfNone(user.userUuid(), user.hash())) {
            return ResponseEntity.accepted().body(user.userUuid().toString());
        } else {
            return ResponseEntity.badRequest().body("Bad Request");
        }
    }
    
    @DeleteMapping("/{uuid}")
    public ResponseEntity<Object> deleteUser(@PathVariable UUID uuid) {
        boolean deleted = deleteUserUseCase.deleteUser(uuid);
        
        if (deleted) {
            return ResponseEntity.accepted().body(deleted);
        } else {
            return ResponseEntity.badRequest().body("Bad Request");
        }
    }
}
