package app.planentnine.springcontinuebee.adapter.web;

import app.planentnine.springcontinuebee.adapter.web.dto.RestHashDto;
import app.planentnine.springcontinuebee.adapter.web.dto.mapper.RestHashDtoMapper;
import app.planentnine.springcontinuebee.application.port.incoming.VerifyHashUseCase;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;

@Controller
public class MessageController {
    
    private final VerifyHashUseCase verifyHashUseCase;
    private final RestHashDtoMapper messageDtoMapper;

    @Autowired
    public MessageController(VerifyHashUseCase verifyHashUseCase,
                             RestHashDtoMapper messageDtoMapper){
        this.verifyHashUseCase = verifyHashUseCase;
        this.messageDtoMapper = messageDtoMapper;
    }
    
    @PostMapping("/do-cool-stuff")
    public ResponseEntity<String> verifyMessage(@RequestBody RestHashDto messageDto){
        boolean verified = verifyHashUseCase.verifyHash(messageDtoMapper.map(messageDto));
        if (verified) {
            return ResponseEntity.accepted().body("The message content was verified successfully");
        } else {
            return ResponseEntity.badRequest().body("Invalid request parameters provided");
        }
    }
}
