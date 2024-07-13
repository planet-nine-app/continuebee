package app.planentnine.springcontinuebee.adapter.web.dto.mapper;

import app.planentnine.springcontinuebee.adapter.web.dto.RestUserDto;
import app.planentnine.springcontinuebee.application.domain.User;
import org.springframework.stereotype.Component;

@Component
public class RestUserDtoMapper {
    public User map(RestUserDto restUserDto){
        return new User(
                null,
                null,
                restUserDto.pubKey(),
                restUserDto.hash()
        );
    }
}
