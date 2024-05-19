package app.planentnine.springcontinuebee.application.port.incoming;

import app.planentnine.springcontinuebee.application.domain.Hash;

public interface VerifyHashUseCase {
    boolean verifyHash(Hash hash);
}
