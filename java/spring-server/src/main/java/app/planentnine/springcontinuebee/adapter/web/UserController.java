package app.planentnine.springcontinuebee.adapter.web;

import app.planentnine.springcontinuebee.adapter.web.dto.RestHashDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestHashDtoMapper;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestUserDtoMapper;
import app.planentnine.springcontinuebee.application.domain.User;
import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import app.planentnine.springcontinuebee.application.port.incoming.CreateUserUseCase;
import app.planentnine.springcontinuebee.application.port.incoming.VerifyHashUseCase;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;

import java.util.HashMap;
import java.util.Map;


@Controller("/user")
public class UserController {
    
    private final CreateUserUseCase createUserUseCase;
    private final VerifyHashUseCase verifyHashUseCase;
    private final RestUserDtoMapper userDtoMapper;
    private final RestHashDtoMapper hashDtoMapper;
    
    @Autowired
    public UserController(CreateUserUseCase createUserUseCase,
                          VerifyHashUseCase verifyHashUseCase,
                          RestUserDtoMapper restUserDtoMapper,
                          RestHashDtoMapper restHashDtoMapper) {
        this.createUserUseCase = createUserUseCase;
        this.verifyHashUseCase = verifyHashUseCase;
        this.userDtoMapper = restUserDtoMapper;
        this.hashDtoMapper = restHashDtoMapper;
    }
    
    @PutMapping("/{uuid}}")
    public ResponseEntity<Object> createUser(@RequestBody RestUserDto restUserDto) {
        try {
            User user = createUserUseCase.createUser(userDtoMapper.map(restUserDto));
            Map<String, String> responseMap = new HashMap<>();
            responseMap.put("userUuid", user.userUuid().toString());
            return ResponseEntity.accepted().body(responseMap);
        } catch (ValidationException e){
            return ResponseEntity.badRequest().body(e.getErrors());
        }
    }
    
    @GetMapping("/{uuid}")
    public ResponseEntity<String> verifyMessage(@RequestBody RestHashDto messageDto){
        boolean verified = verifyHashUseCase.verifyHash(hashDtoMapper.map(messageDto));
        if (verified) {
            return ResponseEntity.accepted().body("The message content was verified successfully");
        } else {
            return ResponseEntity.badRequest().body("Invalid request parameters provided");
        }
    }
    
}
