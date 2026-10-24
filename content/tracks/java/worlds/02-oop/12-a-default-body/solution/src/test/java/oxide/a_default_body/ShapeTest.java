package oxide.a_default_body;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ShapeTest {
    /** A shape the interface has never heard of. */
    static class Square implements Shape {
        @Override
        public String name() {
            return "Square";
        }

        @Override
        public double area() {
            return 9;
        }
    }

    @Test
    void describeUsesNameAndArea() {
        assertThat(new Rectangle(3, 4).describe()).isEqualTo("Rectangle with area 12");
        assertThat(new Circle(2).describe()).as("12.57 rounds to 13").isEqualTo("Circle with area 13");
    }

    @Test
    void newShapesGetDescribeForFree() {
        assertThat(new Square().describe()).isEqualTo("Square with area 9");
    }
}
