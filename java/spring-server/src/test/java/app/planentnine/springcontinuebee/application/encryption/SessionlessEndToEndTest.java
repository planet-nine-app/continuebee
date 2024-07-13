package app.planentnine.springcontinuebee.application.encryption;

import org.junit.jupiter.api.RepeatedTest;

public class SessionlessEndToEndTest {
    @RepeatedTest(100)
    public void fullSessionlessTest() {
        String[] keys = Sessionless.generateKeys();
        String privateKey = keys[0];
        String publicKey = keys[1];
        
        String message = "Sessionless message";
        
        String signature = Sessionless.sign(privateKey, message);
        
        boolean isVerified = Sessionless.verifySignature(publicKey, signature, message);
        
        assert(isVerified);
    }
}
