package app.planentnine.springcontinuebee.adapter.web.dto.mapper;

import app.planentnine.springcontinuebee.adapter.web.dto.RestCreateUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestDeleteUserDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestUpdateHashDto;
import app.planentnine.springcontinuebee.adapter.web.dto.RestVerifyHashDto;
import app.planentnine.springcontinuebee.application.domain.Message;
import org.springframework.stereotype.Component;

@Component
public class RestMessageMapper {
    public Message map(RestCreateUserDto createUserDto){
        return new Message(
                null,
                createUserDto.hash(),
                String.format("%s%s%s", createUserDto.timestamp(), createUserDto.pubKey(), createUserDto.hash()),
                createUserDto.signature(),
                createUserDto.timestamp()
        );
    }
    
    public Message map(RestDeleteUserDto deleteUserDto){
        return new Message(
                deleteUserDto.userUUID(),
                deleteUserDto.hash(),
                String.format("%s%s%s", deleteUserDto.timestamp(), deleteUserDto.userUUID(), deleteUserDto.hash()),
                deleteUserDto.signature(),
                deleteUserDto.timestamp()
        );
    }
    
    public Message map(RestUpdateHashDto updateHashDto){
        return new Message(
                updateHashDto.userUUID(),
                updateHashDto.hash(),
                String.format("%s%s%s%s",
                        updateHashDto.timestamp(),
                        updateHashDto.userUUID(),
                        updateHashDto.hash(),
                        updateHashDto.newHash()),
                updateHashDto.signature(),
                updateHashDto.timestamp()
        );
    }
    
    public Message map(RestVerifyHashDto verifyHashDto){
        return new Message(
                verifyHashDto.userUUID(),
                verifyHashDto.hash(),
                String.format("%s%s%s", verifyHashDto.timestamp(), verifyHashDto.userUUID(), verifyHashDto.hash()),
                verifyHashDto.signature(),
                verifyHashDto.timestamp()
        );
    }
}
