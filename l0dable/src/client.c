#include <stdio.h>

#define API_ISR(id, isr)
#include "epicardium.h"
#include "api/caller.h"

/* Autogenerated stub for API_INTERRUPT_ENABLE */
int epic_interrupt_enable(api_int_id_t int_id)
{
        const int epc__apistub_size = sizeof(api_int_id_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_INTERRUPT_ENABLE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(api_int_id_t*)(epc__apistub_buffer + 0) = int_id;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_INTERRUPT_DISABLE */
int epic_interrupt_disable(api_int_id_t int_id)
{
        const int epc__apistub_size = sizeof(api_int_id_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_INTERRUPT_DISABLE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(api_int_id_t*)(epc__apistub_buffer + 0) = int_id;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_SYSTEM_EXIT */
void __epic_exit(int ret)
{
        const int epc__apistub_size = sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_SYSTEM_EXIT, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = ret;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_SYSTEM_EXEC */
int __epic_exec(char *name)
{
        const int epc__apistub_size = sizeof(char *);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_SYSTEM_EXEC, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(char **)(epc__apistub_buffer + 0) = name;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_SYSTEM_RESET */
void epic_system_reset(void)
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_SYSTEM_RESET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_BATTERY_VOLTAGE */
int epic_read_battery_voltage(float *result)
{
        const int epc__apistub_size = sizeof(float *);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_BATTERY_VOLTAGE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(float **)(epc__apistub_buffer + 0) = result;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_UART_WRITE_STR */
void epic_uart_write_str( const char *str, intptr_t length )
{
        const int epc__apistub_size = sizeof(const char *) + sizeof(intptr_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_UART_WRITE_STR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char **)(epc__apistub_buffer + 0) = str;
        *(intptr_t*)(epc__apistub_buffer + sizeof(const char *)) = length;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_UART_READ_CHAR */
int epic_uart_read_char(void)
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_UART_READ_CHAR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_UART_READ_STR */
int epic_uart_read_str(char *buf, size_t cnt)
{
        const int epc__apistub_size = sizeof(char *) + sizeof(size_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_UART_READ_STR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(char **)(epc__apistub_buffer + 0) = buf;
        *(size_t*)(epc__apistub_buffer + sizeof(char *)) = cnt;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_BUTTONS_READ */
uint8_t epic_buttons_read(uint8_t mask)
{
        const int epc__apistub_size = sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_BUTTONS_READ, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = mask;

        return *(uint8_t*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_GPIO_SET_PIN_MODE */
int epic_gpio_set_pin_mode(uint8_t pin, uint8_t mode)
{
        const int epc__apistub_size = sizeof(uint8_t) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_GPIO_SET_PIN_MODE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = pin;
        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t)) = mode;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_GPIO_GET_PIN_MODE */
int epic_gpio_get_pin_mode(uint8_t pin)
{
        const int epc__apistub_size = sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_GPIO_GET_PIN_MODE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = pin;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_GPIO_WRITE_PIN */
int epic_gpio_write_pin(uint8_t pin, _Bool on)
{
        const int epc__apistub_size = sizeof(uint8_t) + sizeof(_Bool);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_GPIO_WRITE_PIN, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = pin;
        *(_Bool*)(epc__apistub_buffer + sizeof(uint8_t)) = on;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_GPIO_READ_PIN */
uint32_t epic_gpio_read_pin(uint8_t pin)
{
        const int epc__apistub_size = sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_GPIO_READ_PIN, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = pin;

        return *(uint32_t*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET */
void epic_leds_set(int led, uint8_t r, uint8_t g, uint8_t b)
{
        const int epc__apistub_size = sizeof(int) + sizeof(uint8_t) + sizeof(uint8_t) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = led;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int)) = r;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t)) = g;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t) + sizeof(uint8_t)) = b;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_HSV */
void epic_leds_set_hsv(int led, float h, float s, float v)
{
        const int epc__apistub_size = sizeof(int) + sizeof(float) + sizeof(float) + sizeof(float);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_HSV, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = led;
        *(float*)(epc__apistub_buffer + sizeof(int)) = h;
        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float)) = s;
        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float) + sizeof(float)) = v;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_ALL */
void epic_leds_set_all(uint8_t *pattern, uint8_t len)
{
        const int epc__apistub_size = sizeof(uint8_t *) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_ALL, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t **)(epc__apistub_buffer + 0) = pattern;
        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t *)) = len;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_ALL_HSV */
void epic_leds_set_all_hsv(float *pattern, uint8_t len)
{
        const int epc__apistub_size = sizeof(float *) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_ALL_HSV, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(float **)(epc__apistub_buffer + 0) = pattern;
        *(uint8_t*)(epc__apistub_buffer + sizeof(float *)) = len;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_PREP */
void epic_leds_prep(int led, uint8_t r, uint8_t g, uint8_t b)
{
        const int epc__apistub_size = sizeof(int) + sizeof(uint8_t) + sizeof(uint8_t) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_PREP, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = led;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int)) = r;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t)) = g;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int) + sizeof(uint8_t) + sizeof(uint8_t)) = b;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_PREP_HSV */
void epic_leds_prep_hsv(int led, float h, float s, float v)
{
        const int epc__apistub_size = sizeof(int) + sizeof(float) + sizeof(float) + sizeof(float);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_PREP_HSV, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = led;
        *(float*)(epc__apistub_buffer + sizeof(int)) = h;
        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float)) = s;
        *(float*)(epc__apistub_buffer + sizeof(int) + sizeof(float) + sizeof(float)) = v;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_DIM_BOTTOM */
void epic_leds_dim_bottom(uint8_t value)
{
        const int epc__apistub_size = sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_DIM_BOTTOM, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = value;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_DIM_TOP */
void epic_leds_dim_top(uint8_t value)
{
        const int epc__apistub_size = sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_DIM_TOP, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = value;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_POWERSAVE */
void epic_leds_set_powersave(_Bool eco)
{
        const int epc__apistub_size = sizeof(_Bool);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_POWERSAVE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(_Bool*)(epc__apistub_buffer + 0) = eco;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_UPDATE */
void epic_leds_update(void)
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_UPDATE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_ROCKET */
void epic_leds_set_rocket(int led, uint8_t value)
{
        const int epc__apistub_size = sizeof(int) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_ROCKET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = led;
        *(uint8_t*)(epc__apistub_buffer + sizeof(int)) = value;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_FLASHLIGHT */
void epic_set_flashlight(_Bool power)
{
        const int epc__apistub_size = sizeof(_Bool);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_FLASHLIGHT, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(_Bool*)(epc__apistub_buffer + 0) = power;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_SET_GAMMA_TABLE */
void epic_leds_set_gamma_table( uint8_t rgb_channel, uint8_t *gamma_table )
{
        const int epc__apistub_size = sizeof(uint8_t) + sizeof(uint8_t *);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_SET_GAMMA_TABLE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = rgb_channel;
        *(uint8_t **)(epc__apistub_buffer + sizeof(uint8_t)) = gamma_table;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LEDS_CLEAR_ALL */
void epic_leds_clear_all(uint8_t r, uint8_t g, uint8_t b)
{
        const int epc__apistub_size = sizeof(uint8_t) + sizeof(uint8_t) + sizeof(uint8_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LEDS_CLEAR_ALL, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = r;
        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t)) = g;
        *(uint8_t*)(epc__apistub_buffer + sizeof(uint8_t) + sizeof(uint8_t)) = b;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_PERSONAL_STATE_SET */
int epic_personal_state_set(uint8_t state, _Bool persistent)
{
        const int epc__apistub_size = sizeof(uint8_t) + sizeof(_Bool);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_PERSONAL_STATE_SET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t*)(epc__apistub_buffer + 0) = state;
        *(_Bool*)(epc__apistub_buffer + sizeof(uint8_t)) = persistent;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_PERSONAL_STATE_GET */
int epic_personal_state_get()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_PERSONAL_STATE_GET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_PERSONAL_STATE_IS_PERSISTENT */
int epic_personal_state_is_persistent()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_PERSONAL_STATE_IS_PERSISTENT, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_STREAM_READ */
int epic_stream_read(int sd, void *buf, size_t count)
{
        const int epc__apistub_size = sizeof(int) + sizeof(void *) + sizeof(size_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_STREAM_READ, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = sd;
        *(void **)(epc__apistub_buffer + sizeof(int)) = buf;
        *(size_t*)(epc__apistub_buffer + sizeof(int) + sizeof(void *)) = count;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_VIBRA_SET */
void epic_vibra_set(int status)
{
        const int epc__apistub_size = sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_VIBRA_SET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = status;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_VIBRA_VIBRATE */
void epic_vibra_vibrate(int millis)
{
        const int epc__apistub_size = sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_VIBRA_VIBRATE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = millis;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_OPEN */
int epic_disp_open()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_OPEN, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_CLOSE */
int epic_disp_close()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_CLOSE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_UPDATE */
int epic_disp_update()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_UPDATE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_PRINT */
int epic_disp_print( uint16_t posx, uint16_t posy, const char *pString, uint16_t fg, uint16_t bg)
{
        const int epc__apistub_size = sizeof(uint16_t) + sizeof(uint16_t) + sizeof(const char *) + sizeof(uint16_t) + sizeof(uint16_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_PRINT, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t*)(epc__apistub_buffer + 0) = posx;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)) = posy;
        *(const char **)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)) = pString;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(const char *)) = fg;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(const char *) + sizeof(uint16_t)) = bg;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_CLEAR */
int epic_disp_clear(uint16_t color)
{
        const int epc__apistub_size = sizeof(uint16_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_CLEAR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t*)(epc__apistub_buffer + 0) = color;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_PIXEL */
int epic_disp_pixel( uint16_t x, uint16_t y, uint16_t color)
{
        const int epc__apistub_size = sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_PIXEL, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t*)(epc__apistub_buffer + 0) = x;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)) = y;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)) = color;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_LINE */
int epic_disp_line( uint16_t xstart, uint16_t ystart, uint16_t xend, uint16_t yend, uint16_t color, enum disp_linestyle linestyle, uint16_t pixelsize)
{
        const int epc__apistub_size = sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_linestyle) + sizeof(uint16_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_LINE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t*)(epc__apistub_buffer + 0) = xstart;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)) = ystart;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)) = xend;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = yend;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = color;
        *(enum disp_linestyle*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = linestyle;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_linestyle)) = pixelsize;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_RECT */
int epic_disp_rect( uint16_t xstart, uint16_t ystart, uint16_t xend, uint16_t yend, uint16_t color, enum disp_fillstyle fillstyle, uint16_t pixelsize)
{
        const int epc__apistub_size = sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_fillstyle) + sizeof(uint16_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_RECT, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t*)(epc__apistub_buffer + 0) = xstart;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)) = ystart;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)) = xend;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = yend;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = color;
        *(enum disp_fillstyle*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = fillstyle;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_fillstyle)) = pixelsize;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_CIRC */
int epic_disp_circ( uint16_t x, uint16_t y, uint16_t rad, uint16_t color, enum disp_fillstyle fillstyle, uint16_t pixelsize)
{
        const int epc__apistub_size = sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_fillstyle) + sizeof(uint16_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_CIRC, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t*)(epc__apistub_buffer + 0) = x;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t)) = y;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t)) = rad;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = color;
        *(enum disp_fillstyle*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t)) = fillstyle;
        *(uint16_t*)(epc__apistub_buffer + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(uint16_t) + sizeof(enum disp_fillstyle)) = pixelsize;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_DISP_FRAMEBUFFER */
int epic_disp_framebuffer(union disp_framebuffer *fb)
{
        const int epc__apistub_size = sizeof(union disp_framebuffer *);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_DISP_FRAMEBUFFER, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(union disp_framebuffer **)(epc__apistub_buffer + 0) = fb;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LIGHT_SENSOR_RUN */
int epic_light_sensor_run()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LIGHT_SENSOR_RUN, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LIGHT_SENSOR_GET */
int epic_light_sensor_get(uint16_t* value)
{
        const int epc__apistub_size = sizeof(uint16_t*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LIGHT_SENSOR_GET, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint16_t**)(epc__apistub_buffer + 0) = value;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_LIGHT_SENSOR_STOP */
int epic_light_sensor_stop()
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_LIGHT_SENSOR_STOP, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_OPEN */
int epic_file_open( const char* filename, const char* modeString )
{
        const int epc__apistub_size = sizeof(const char*) + sizeof(const char*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_OPEN, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char**)(epc__apistub_buffer + 0) = filename;
        *(const char**)(epc__apistub_buffer + sizeof(const char*)) = modeString;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_CLOSE */
int epic_file_close(int fd)
{
        const int epc__apistub_size = sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_CLOSE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_READ */
int epic_file_read(int fd, void* buf, size_t nbytes)
{
        const int epc__apistub_size = sizeof(int) + sizeof(void*) + sizeof(size_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_READ, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;
        *(void**)(epc__apistub_buffer + sizeof(int)) = buf;
        *(size_t*)(epc__apistub_buffer + sizeof(int) + sizeof(void*)) = nbytes;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_WRITE */
int epic_file_write(int fd, const void* buf, size_t nbytes)
{
        const int epc__apistub_size = sizeof(int) + sizeof(const void*) + sizeof(size_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_WRITE, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;
        *(const void**)(epc__apistub_buffer + sizeof(int)) = buf;
        *(size_t*)(epc__apistub_buffer + sizeof(int) + sizeof(const void*)) = nbytes;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_FLUSH */
int epic_file_flush(int fd)
{
        const int epc__apistub_size = sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_FLUSH, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_SEEK */
int epic_file_seek(int fd, long offset, int whence)
{
        const int epc__apistub_size = sizeof(int) + sizeof(long) + sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_SEEK, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;
        *(long*)(epc__apistub_buffer + sizeof(int)) = offset;
        *(int*)(epc__apistub_buffer + sizeof(int) + sizeof(long)) = whence;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_TELL */
int epic_file_tell(int fd)
{
        const int epc__apistub_size = sizeof(int);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_TELL, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_STAT */
int epic_file_stat( const char* path, struct epic_stat* stat )
{
        const int epc__apistub_size = sizeof(const char*) + sizeof(struct epic_stat*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_STAT, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char**)(epc__apistub_buffer + 0) = path;
        *(struct epic_stat**)(epc__apistub_buffer + sizeof(const char*)) = stat;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_OPENDIR */
int epic_file_opendir(const char* path)
{
        const int epc__apistub_size = sizeof(const char*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_OPENDIR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char**)(epc__apistub_buffer + 0) = path;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_READDIR */
int epic_file_readdir(int fd, struct epic_stat* stat)
{
        const int epc__apistub_size = sizeof(int) + sizeof(struct epic_stat*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_READDIR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(int*)(epc__apistub_buffer + 0) = fd;
        *(struct epic_stat**)(epc__apistub_buffer + sizeof(int)) = stat;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_UNLINK */
int epic_file_unlink(const char* path)
{
        const int epc__apistub_size = sizeof(const char*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_UNLINK, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char**)(epc__apistub_buffer + 0) = path;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_RENAME */
int epic_file_rename(const char *oldp, const char* newp)
{
        const int epc__apistub_size = sizeof(const char *) + sizeof(const char*);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_RENAME, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char **)(epc__apistub_buffer + 0) = oldp;
        *(const char**)(epc__apistub_buffer + sizeof(const char *)) = newp;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_FILE_MKDIR */
int epic_file_mkdir(const char *dirname)
{
        const int epc__apistub_size = sizeof(const char *);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_FILE_MKDIR, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(const char **)(epc__apistub_buffer + 0) = dirname;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_RTC_GET_SECONDS */
uint32_t epic_rtc_get_seconds(void)
{
        const int epc__apistub_size = 0;
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_RTC_GET_SECONDS, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */


        return *(uint32_t*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_RTC_SET_MILLISECONDS */
void epic_rtc_set_milliseconds(uint64_t milliseconds)
{
        const int epc__apistub_size = sizeof(uint64_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_RTC_SET_MILLISECONDS, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint64_t*)(epc__apistub_buffer + 0) = milliseconds;

        _api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_RTC_SCHEDULE_ALARM */
int epic_rtc_schedule_alarm(uint32_t timestamp)
{
        const int epc__apistub_size = sizeof(uint32_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_RTC_SCHEDULE_ALARM, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint32_t*)(epc__apistub_buffer + 0) = timestamp;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}

/* Autogenerated stub for API_TRNG_READ */
int epic_trng_read(uint8_t *dest, size_t size)
{
        const int epc__apistub_size = sizeof(uint8_t *) + sizeof(size_t);
        void*epc__apistub_buffer;

        epc__apistub_buffer = _api_call_start(API_TRNG_READ, epc__apistub_size);
        /* TODO: Check if epc__apistub_buffer is not NULL */

        *(uint8_t **)(epc__apistub_buffer + 0) = dest;
        *(size_t*)(epc__apistub_buffer + sizeof(uint8_t *)) = size;

        return *(int*)_api_call_transact(epc__apistub_buffer);
}


/* Weakly linked stubs for ISRs */
void __epic_isr_reset(api_int_id_t id)
        __attribute__((weak, alias("__epic_isr_default_handler")));
void epic_isr_uart_rx(api_int_id_t id)
        __attribute__((weak, alias("__epic_isr_default_handler")));
void epic_isr_ctrl_c(api_int_id_t id)
        __attribute__((weak, alias("__epic_isr_default_handler")));
void epic_isr_rtc_alarm(api_int_id_t id)
        __attribute__((weak, alias("__epic_isr_default_handler")));

/* Default handler stub */
__attribute__((weak)) void epic_isr_default_handler(api_int_id_t id)
{
        ;
}

/*
 * This function is needed because aliasing the weak default handler will
 * lead to issues.
 */
void __epic_isr_default_handler(api_int_id_t id)
{
        epic_isr_default_handler(id);
}

/*
 * __dispatch_isr() will be called from the actual isr which was triggered
 * by core 0.  It will then call the appropriate isr.
 */
void __dispatch_isr(api_int_id_t id)
{
        switch (id) {
        case EPIC_INT_RESET:
                __epic_isr_reset(id);
                break;
        case EPIC_INT_UART_RX:
                epic_isr_uart_rx(id);
                break;
        case EPIC_INT_CTRL_C:
                epic_isr_ctrl_c(id);
                break;
        case EPIC_INT_RTC_ALARM:
                epic_isr_rtc_alarm(id);
                break;
        case (-1):
                /* Ignore a spurious interrupt */
                break;
        default:
                epic_isr_default_handler(id);
                break;
        }
}