/**
 */


#include <U8g2lib.h>
#include <Wire.h>

#if defined(ARDUINO_ARCH_AVR)
    #define debug  Serial

#elif defined(ARDUINO_ARCH_SAMD) ||  defined(ARDUINO_ARCH_SAM)
    #define debug  SerialUSB
#else
    #define debug  Serial
#endif

constexpr const unsigned long SERIAL_BAUD_RATE = 115200;
constexpr const unsigned long SHOW_TIME_DELAY_MS = 2000;

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2(U8G2_R0, /* reset=*/ U8X8_PIN_NONE);

void setup() {
    debug.begin(SERIAL_BAUD_RATE);
    u8g2.begin();
}

int COUNTER = 0;

void loop() {
    u8g2.clearBuffer();                  // clear the internal memory
    u8g2.setFont(u8g2_font_ncenB08_tr);  // choose a suitable font
    u8g2.drawStr(0, 10, "Hello World!"); // write something to the internal memory
    u8g2.setCursor(0,50);
    u8g2.print(COUNTER);
    u8g2.sendBuffer();
    COUNTER++;
    delay(SHOW_TIME_DELAY_MS);
}