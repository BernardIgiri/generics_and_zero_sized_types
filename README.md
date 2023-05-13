# Experiment with generics_and_zero_sized_types
## from https://youtu.be/_ccDqRTx-JU

Let's Get Rusty's video on zero sized types was very informative and did a great job of demonstrating what is possible with that feature. However, in evaluating that example I found that this feature doesn't apply very well which leads me to suspect that finding a practical use case for it is not trivial.

Regardless, I experimented with the example from the video to see if using a zero sized type is really a good fit for a password manager using RAII on it's lock and unlock feature. To investigate this, I tried implmenting a similar password manager to see just how similar locked and unlock structs really needed to be. I figured that if they were dissimilar enough, then the use of a zero sized type would not be effective, as the objects would differ enough on there own to not need it.

My first step was to created a "niave implementation" where instead of the zero sized type, I simply pass ownership of the locked vault to the unlocked vault. Doing this, I was able to implement all of the necessary functionality without using a zero sized type. Taking this experiment a step further with rudimentary encryption, I found that the unlocked password manager only needed the master password and a decrypted password map from the locked one. The locked one generates this decrypted map on unlock and does not store it. The master password only needs to persist in RAM on unlock, and is only needed to support reencryption on lock.

Thus, there is no explicit need to share data in the same format between the two. However, in a practical real world application there may be some minor data sharing between both objects. Regardless, the differences in data needs for both objects appears large enough that using a zero sized type as differentiator appears redundant.

Also note, I renamed the PasswordManager to PasswordVault.