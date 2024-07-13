package app.planentnine.springcontinuebee.application.port.incoming;

import app.planentnine.springcontinuebee.application.domain.Message;
import app.planentnine.springcontinuebee.application.domain.User;

public interface CreateUserUseCase {
    User createUser(Message message, User user);
}
