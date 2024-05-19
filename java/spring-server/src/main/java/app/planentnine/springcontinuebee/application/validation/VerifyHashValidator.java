package app.planentnine.springcontinuebee.application.validation;

import app.planentnine.springcontinuebee.application.domain.exception.ValidationException;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneOffset;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

@Component
public class VerifyHashValidator {
    
    @Value("${hash.valid-window-in-seconds}")
    private int VALID_WINDOW_IN_SECONDS;
    
    public Optional<ValidationException> validate(LocalDateTime timestamp){
        List<String> errors = new ArrayList<>();
        
        if (timestamp.isBefore(
                LocalDateTime.ofInstant(Instant.now(), ZoneOffset.UTC)
                .minusSeconds(VALID_WINDOW_IN_SECONDS))){
            
            errors.add("Timestamp only valid within " + VALID_WINDOW_IN_SECONDS + " seconds");
            errors.add("Now: " + LocalDateTime.now());
            errors.add("Provided: " + timestamp);
        }
        
        if (errors.isEmpty()){
            return Optional.empty();
        } else {
            return Optional.of(new ValidationException(errors));
        }
        
    }
}
