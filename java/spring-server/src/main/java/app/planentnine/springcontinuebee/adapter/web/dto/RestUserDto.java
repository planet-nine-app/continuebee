package app.planentnine.springcontinuebee.adapter.web.dto;

import lombok.Builder;
import lombok.extern.jackson.Jacksonized;

@Builder
@Jacksonized
public record RestUserDto(String uuid, String pubKey, String hash) {
}
