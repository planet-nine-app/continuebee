package app.planentnine.springcontinuebee.application.port.incoming;

import app.planentnine.springcontinuebee.application.domain.User;

public interface CreateUserUseCase {
    User createUser(User user);
}
