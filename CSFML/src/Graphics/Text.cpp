
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

// Headers

#include "Graphics/Color.h"
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Text.hpp>
#include <cstddef>

extern "C" sf::Text *sfText_create(void) {
    sf::Text *text = new sf::Text;

    return text;
}

extern "C" sf::Text *sfText_copy(const sf::Text *text) {
    return new sf::Text(*text);
}

extern "C" void sfText_destroy(sf::Text *text) {
    delete text;
}

extern "C" void sfText_setPosition(sf::Text *text, sfVector2f position) {
    text->setPosition(position.x, position.y);
}

extern "C" void sfText_setRotation(sf::Text *text, float angle) {
    text->setRotation(angle);
}

extern "C" void sfText_setScale(sf::Text *text, sfVector2f scale) {
    text->setScale(scale.x, scale.y);
}

extern "C" void sfText_setOrigin(sf::Text *text, sfVector2f origin) {
    text->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfText_getPosition(const sf::Text *text) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = text->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfText_getRotation(const sf::Text *text) {
    return text->getRotation();
}

extern "C" sfVector2f sfText_getScale(const sf::Text *text) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = text->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfText_getOrigin(const sf::Text *text) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = text->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfText_move(sf::Text *text, sfVector2f offset) {
    text->move(offset.x, offset.y);
}

extern "C" void sfText_rotate(sf::Text *text, float angle) {
    text->rotate(angle);
}

extern "C" void sfText_scale(sf::Text *text, sfVector2f factors) {
    text->scale(factors.x, factors.y);
}

extern "C" sf::Transform const *sfText_getTransform(const sf::Text *text) {
    return &text->getTransform();
}

extern "C" sf::Transform const *sfText_getInverseTransform(const sf::Text *text) {
    return &text->getInverseTransform();
}

extern "C" void sfText_setUnicodeString(sf::Text *text, const uint32_t *string) {
    sf::String UTF32Text = string;
    text->setString(UTF32Text);
}

extern "C" void sfText_setFont(sf::Text *text, const sf::Font *font) {

    text->setFont(*font);
}

extern "C" void sfText_setCharacterSize(sf::Text *text, unsigned int size) {
    text->setCharacterSize(size);
}

extern "C" void sfText_setLineSpacing(sf::Text *text, float spacingFactor) {
    text->setLineSpacing(spacingFactor);
}

extern "C" void sfText_setLetterSpacing(sf::Text *text, float spacingFactor) {
    text->setLetterSpacing(spacingFactor);
}

extern "C" void sfText_setStyle(sf::Text *text, uint32_t style) {
    text->setStyle(style);
}

extern "C" void sfText_setFillColor(sf::Text *text, sfColor color) {
    text->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfText_setOutlineColor(sf::Text *text, sfColor color) {
    text->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfText_setOutlineThickness(sf::Text *text, float thickness) {
    text->setOutlineThickness(thickness);
}

extern "C" const uint32_t *sfText_getUnicodeString(const sf::Text *text) {

    return text->getString().getData();
}

extern "C" const sf::Font *sfText_getFont(const sf::Text *text) {
    const sf::Font *font = text->getFont();
    return font;
}

extern "C" unsigned int sfText_getCharacterSize(const sf::Text *text) {
    return text->getCharacterSize();
}

extern "C" float sfText_getLetterSpacing(const sf::Text *text) {
    return text->getLetterSpacing();
}

extern "C" float sfText_getLineSpacing(const sf::Text *text) {
    return text->getLineSpacing();
}

extern "C" uint32_t sfText_getStyle(const sf::Text *text) {
    return text->getStyle();
}

extern "C" sfColor sfText_getFillColor(const sf::Text *text) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = text->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfText_getOutlineColor(const sf::Text *text) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = text->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfText_getOutlineThickness(const sf::Text *text) {
    return text->getOutlineThickness();
}

extern "C" sfVector2f sfText_findCharacterPos(const sf::Text *text, size_t index) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = text->findCharacterPos(index);
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" sfFloatRect sfText_getLocalBounds(const sf::Text *text) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = text->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfText_getGlobalBounds(const sf::Text *text) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = text->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
