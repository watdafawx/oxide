package oxide.main_and_args;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BannerTest {
    @Test
    void bannerUsesFirstArg() {
        assertThat(Banner.banner(new String[] {"Ada"})).isEqualTo("Hello, Ada!");
        assertThat(Banner.banner(new String[] {"Grace", "ignored"})).isEqualTo("Hello, Grace!");
    }

    @Test
    void bannerDefaultsWithoutArgs() {
        assertThat(Banner.banner(new String[] {})).isEqualTo("Hello, Oxide!");
    }
}
