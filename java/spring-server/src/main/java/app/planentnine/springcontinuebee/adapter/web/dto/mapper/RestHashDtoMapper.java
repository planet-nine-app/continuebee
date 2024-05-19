package app.planentnine.springcontinuebee.adapter.web.dto.mapper;

import app.planentnine.springcontinuebee.adapter.web.dto.RestHashDto;
import app.planentnine.springcontinuebee.application.domain.Hash;
import org.springframework.stereotype.Component;

@Component
public class RestHashDtoMapper {
    public Hash map(RestHashDto restHashDto){
        return new Hash(
                restHashDto.uuid(),
                restHashDto.hash(),
                restHashDto.signature(),
                restHashDto.timestamp()
        );
    }
}
