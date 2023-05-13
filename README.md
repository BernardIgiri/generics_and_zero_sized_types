# Experiment with generics_and_zero_sized_types
## from https://youtu.be/_ccDqRTx-JU

I experimented with the example of using generics and zero sized types just to see how else I could get the intended functionality of the password manager in the example video. To my eye, it doesn't seem that a password manager is a great use case for this zero sized types feature. I suspect that finding a better use case is difficult. This feature appears to be incredibly niche.

Regardless, to investigate why I feel this password manager is not a great use case, I tried implmenting a similar password manager and I found that the data needed by the locked and unlocked versions of are not similar at all. First I created a "niave implementation" where instead of the zero sized type, I simply pass ownership of the locked vault to the unlocked vault. Doing this, I was able to implement all of the necessary functionality without using a zero sized type.

Taking this experiment a step further with rudimentary encryption, I found that the unlocked password vault only needed the master password and a decrypted password map from the locked vault. The locked vault generates this decrypted map on unlock and does not store it. The master password only needs to persist in RAM on unlock, and is only needed to support reencryption on lock.

Thus, there is no explicit need to share data in the same format between the two. However, in a practical real world application there may be some minor data sharing between both objects. Regardless, the differences in data needs for both objects appears large enough to invalidate the benefits of using a zero sized type as the differentiating value.