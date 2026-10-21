package oxide.main_and_args;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BannerTest {
    @Test
    void bannerUsesFirstArg() {
        assertThat(Banner.banner(new String[] {"Ada"})).isEqualTo("Hello, Ada!");
        assertThat(Banner.banner(new String[] {"Bo", "extra"})).isEqualTo("Hello, Bo!");
    }

    @Test
    void pairBannerUsesTwoArgs() {
        assertThat(Banner.pairBanner(new String[] {"Ada", "Bo"})).isEqualTo("Hello, Ada and Bo!");
    }
}
