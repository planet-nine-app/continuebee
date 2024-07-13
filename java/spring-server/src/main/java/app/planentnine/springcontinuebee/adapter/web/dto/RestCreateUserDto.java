package app.planentnine.springcontinuebee.adapter.web.dto;

import lombok.Builder;
import lombok.extern.jackson.Jacksonized;

@Builder
@Jacksonized
public record RestCreateUserDto(long timestamp, String pubKey, String hash, String signature) {
}
