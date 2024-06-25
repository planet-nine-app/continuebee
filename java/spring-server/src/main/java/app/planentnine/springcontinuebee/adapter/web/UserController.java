package app.planentnine.springcontinuebee.adapter.web;

import app.planentnine.springcontinuebee.adapter.web.dto.RestCreateUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestDeleteUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestUpdateHashDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestVerifyHashDto;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestMessageMapper;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestUserDtoMapper;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.port.incoming.CreateUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.DeleteUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.UpdateHashUseCase;
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

import java.util.HashMap;
import java.util.Map;
import java.util.UUID;


@Controller("/user")
public class UserController {
    
    private final CreateUserUseCase createUserUseCase;
    private final DeleteUserUseCase deleteUserUseCase;
    private final UpdateHashUseCase updateHashUseCase;
    private final VerifyHashUseCase verifyHashUseCase;
    private final RestUserDtoMapper userDtoMapper;
    private final RestMessageMapper messageMapper;
    
    @Autowired
    public UserController(CreateUserUseCase createUserUseCase,
                          DeleteUserUseCase deleteUserUseCase,
                          UpdateHashUseCase updateHashUseCase,
                          VerifyHashUseCase verifyHashUseCase,
                          RestUserDtoMapper restUserDtoMapper,
                          RestMessageMapper messageMapper) {
        this.createUserUseCase = createUserUseCase;
        this.deleteUserUseCase = deleteUserUseCase;
        this.updateHashUseCase = updateHashUseCase;
        this.verifyHashUseCase = verifyHashUseCase;
        this.userDtoMapper = restUserDtoMapper;
        this.messageMapper = messageMapper;
    }
    
    @PutMapping("/create")
    public ResponseEntity<Object> createUser(@RequestBody RestCreateUserDto createUserDto) {
        try {
            RestUserDto userDto = RestUserDto.builder()
                    .userUUID(null)
                    .pubKey(createUserDto.pubKey())
                    .hash(createUserDto.hash())
                    .build();
            
            
            User user = createUserUseCase.createUser(messageMapper.map(createUserDto), userDtoMapper.map(userDto));
            Map<String, String> responseMap = new HashMap<>();
            responseMap.put("userUuid", user.userUuid().toString());
            return ResponseEntity.accepted().body(responseMap);
        } catch (ValidationException e) {
            return ResponseEntity.badRequest().body(e.getErrors());
        }
    }
    
    @GetMapping("/{uuid}")
    public ResponseEntity<Object> verifyHash(@PathVariable UUID uuid,
                                             @RequestParam Long timestamp,
                                             @RequestParam String hash,
                                             @RequestParam String signature) {
        try {
             RestVerifyHashDto verifyHashDto = new RestVerifyHashDto(
                     timestamp,
                     uuid,
                     hash,
                     signature
             );
            
            boolean verified = verifyHashUseCase.verifyHash(messageMapper.map(verifyHashDto));
            if (verified) {
                return ResponseEntity.accepted().body(uuid.toString());
            } else {
                return ResponseEntity.badRequest().body("Bad Request");
            }
        } catch (ValidationException e) {
            return ResponseEntity.badRequest().body(e.getErrors());
        }
    }
    
    @PostMapping("/update-hash")
    public ResponseEntity<Object> updateHash(@RequestBody RestUpdateHashDto updateHashDto) {
        if (updateHashUseCase.updateHash(messageMapper.map(updateHashDto), updateHashDto.newHash())) {
            return ResponseEntity.accepted().body(updateHashDto.userUuid().toString());
        } else {
            return ResponseEntity.badRequest().body("Bad Request");
        }
    }
    
    @DeleteMapping("/delete")
    public ResponseEntity<Object> deleteUser(@RequestBody RestDeleteUserDto deleteUserDto) {
        boolean deleted = deleteUserUseCase.deleteUser(messageMapper.map(deleteUserDto));
        
        if (deleted) {
            return ResponseEntity.accepted().body(true);
        } else {
            return ResponseEntity.badRequest().body("Bad Request");
        }
    }
}
