/**
 * @file dht22_sensor_read_temp.ino
 * @brief Based on the example from Grove_Temperature_And_Humidity_Sensor Arduino library:
 *      https://github.com/Seeed-Studio/Grove_Temperature_And_Humidity_Sensor/blob/master/examples/DHTtester/DHTtester.ino
 */


#include <U8g2lib.h> // OLED
#include <Wire.h> // SPI

#include "DHT.h"

#define DHTTYPE DHT22   // DHT 22  (AM2302)

#define DHTPIN 7
DHT dht(DHTPIN, DHTTYPE);

#if defined(ARDUINO_ARCH_AVR)
    #define debug  Serial

#elif defined(ARDUINO_ARCH_SAMD) ||  defined(ARDUINO_ARCH_SAM)
    #define debug  SerialUSB
#else
    #define debug  Serial
#endif

constexpr const unsigned long SERIAL_BAUD_RATE = 115200;
constexpr const unsigned long MEASUREMENT_DELAY_MS = 20000;

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2(U8G2_R0, /* reset=*/ U8X8_PIN_NONE);

void setup() {
    debug.begin(SERIAL_BAUD_RATE);
    dht.begin();
    u8g2.begin();
}

// Reading temperature or humidity takes about 250 milliseconds!
// Sensor readings may also be up to 2 seconds 'old' (its a very slow sensor)
void loop() {
    float temp_hum_val[2] = {0};
    if (!dht.readTempAndHumidity(temp_hum_val))
    {
        u8g2.clearBuffer();                 // clear the internal memory
        u8g2.setFont(u8g2_font_ncenB08_tr); // choose a suitable font

        String tempStr = "Temperature: " + String(temp_hum_val[1]) + " *C";
        debug.println(tempStr);
        u8g2.drawStr(0, 10, tempStr.c_str());

        String humidityStr = "Humidity: " + String(temp_hum_val[0]) + " %";
        debug.println(humidityStr);
        u8g2.drawStr(0, 25, humidityStr.c_str());

        u8g2.sendBuffer();
    } else {
        debug.println("Failed to get temprature and humidity value.");
    }

    delay(MEASUREMENT_DELAY_MS);
}