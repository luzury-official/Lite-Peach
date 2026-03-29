#include <SFML/Graphics.hpp>

int main() {
    sf::RenderWindow window(sf::VideoMode(720, 1200), "Lite-Peach Test");

    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed)
                window.close();
        }

        window.clear(sf::Color(255, 255, 255));
        window.display();
    }
    return 0;
}