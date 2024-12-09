/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : main.c
  * @brief          : Main program body
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2024 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */
/* Includes ------------------------------------------------------------------*/
#include "main.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */
#include <string.h>

#include "fonts.h"
#include "ST7735.h"
#include "testimg.h"
/* USER CODE END Includes */

/* Private typedef -----------------------------------------------------------*/
/* USER CODE BEGIN PTD */

/* USER CODE END PTD */

/* Private define ------------------------------------------------------------*/
/* USER CODE BEGIN PD */

/* USER CODE END PD */

/* Private macro -------------------------------------------------------------*/
/* USER CODE BEGIN PM */

/* USER CODE END PM */

/* Private variables ---------------------------------------------------------*/

COM_InitTypeDef BspCOMInit;
__IO uint32_t BspButtonState = BUTTON_RELEASED;

UART_HandleTypeDef hlpuart1;

SPI_HandleTypeDef hspi1;

/* USER CODE BEGIN PV */

/* USER CODE END PV */

/* Private function prototypes -----------------------------------------------*/
void SystemClock_Config(void);
static void MX_GPIO_Init(void);
static void MX_LPUART1_UART_Init(void);
static void MX_SPI1_Init(void);
/* USER CODE BEGIN PFP */

/* USER CODE END PFP */

/* Private user code ---------------------------------------------------------*/
/* USER CODE BEGIN 0 */
uint8_t r = 1;
void demoTFT(void)
{
	ST7735_SetRotation(r);
	ST7735_FillScreen(ST7735_BLACK);

	for(int x = 0; x < ST7735_GetWidth(); x++)
	{
		ST7735_DrawPixel(x, 0, ST7735_WHITE);
		ST7735_DrawPixel(x, ST7735_GetHeight() - 1, ST7735_WHITE);
	}

	for(int y = 0; y < ST7735_GetHeight(); y++)
	{
		ST7735_DrawPixel(0, y, ST7735_WHITE);
		ST7735_DrawPixel(ST7735_GetWidth() - 1, y, ST7735_WHITE);
	}

	ST7735_DrawLine(0, 0, ST7735_GetWidth(), ST7735_GetHeight(),ST7735_WHITE);
	ST7735_DrawLine(ST7735_GetWidth(), 0, 0, ST7735_GetHeight(),ST7735_WHITE);
	HAL_Delay(2000);

	ST7735_FillScreen(ST7735_BLACK);
	for (int i = 0; i < ST7735_GetHeight(); i += 4)
	{
		ST7735_DrawFastHLine(0, i, ST7735_GetWidth() - 1, ST7735_WHITE);
	}

	for (int i = 0; i < ST7735_GetWidth(); i += 4)
	{
		ST7735_DrawFastVLine(i, 0, ST7735_GetHeight() - 1, ST7735_WHITE);
	}
	HAL_Delay(2000);

	 // Check fonts
	ST7735_FillScreen(ST7735_BLACK);
	ST7735_DrawString(0, 0,"Font_7x10, red on black,ABCDEF",Font_7x10,ST7735_RED, ST7735_BLACK);
	ST7735_DrawString(0, 3*10, "Font_11x18, green,ABCD",Font_11x18,ST7735_GREEN, ST7735_BLACK);
	ST7735_DrawString(0, 3*10+3*18, "Font_16x26 ,AB", Font_16x26, ST7735_BLUE, ST7735_BLACK);
	HAL_Delay(2000);

	 // Check colors
	ST7735_FillScreen(ST7735_BLACK);
	ST7735_DrawString(0, 0, "BLACK", Font_11x18, ST7735_WHITE, ST7735_BLACK);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_BLUE);
	ST7735_DrawString(0, 0, "BLUE", Font_11x18, ST7735_BLACK, ST7735_BLUE);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_RED);
	ST7735_DrawString(0, 0, "RED", Font_11x18, ST7735_BLACK, ST7735_RED);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_GREEN);
	ST7735_DrawString(0, 0, "GREEN", Font_11x18, ST7735_BLACK, ST7735_GREEN);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_CYAN);
	ST7735_DrawString(0, 0, "CYAN", Font_11x18, ST7735_BLACK, ST7735_CYAN);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_MAGENTA);
	ST7735_DrawString(0, 0, "MAGENTA",Font_11x18, ST7735_BLACK, ST7735_MAGENTA);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_YELLOW);
	ST7735_DrawString(0, 0, "YELLOW", Font_11x18, ST7735_BLACK, ST7735_YELLOW);
	HAL_Delay(500);

	ST7735_FillScreen(ST7735_WHITE);
	ST7735_DrawString(0, 0, "WHITE", Font_11x18, ST7735_BLACK, ST7735_WHITE);
	HAL_Delay(500);

	// Draw circles
	ST7735_FillScreen(ST7735_BLACK);
	for (int i = 0; i < ST7735_GetHeight() / 2; i += 2)
	{
		ST7735_DrawCircle(ST7735_GetWidth()/2,ST7735_GetHeight()/2,i,ST7735_YELLOW);
	}
	HAL_Delay(1000);

	ST7735_FillScreen(ST7735_BLACK);
	ST7735_FillTriangle(0, 0, ST7735_GetWidth() / 2, ST7735_GetHeight(),
	ST7735_GetWidth(), 0, ST7735_RED);
	HAL_Delay(1000);

	ST7735_DrawImage(0,0, 128, 90, (uint16_t*) test_img_128x128);
	HAL_Delay(3000);
	ST7735_FillScreen(ST7735_WHITE);
}
/* USER CODE END 0 */

/**
  * @brief  The application entry point.
  * @retval int
  */
int main(void)
{

  /* USER CODE BEGIN 1 */

  /* USER CODE END 1 */

  /* MCU Configuration--------------------------------------------------------*/

  /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
  HAL_Init();

  /* USER CODE BEGIN Init */

  /* USER CODE END Init */

  /* Configure the system clock */
  SystemClock_Config();

  /* USER CODE BEGIN SysInit */

  /* USER CODE END SysInit */

  /* Initialize all configured peripherals */
  MX_GPIO_Init();
  MX_LPUART1_UART_Init();
  MX_SPI1_Init();
  /* USER CODE BEGIN 2 */

  /* USER CODE END 2 */

  /* Initialize USER push-button, will be used to trigger an interrupt each time it's pressed.*/
  BSP_PB_Init(BUTTON_USER, BUTTON_MODE_EXTI);

  /* Initialize COM1 port (115200, 8 bits (7-bit data + 1 stop bit), no parity */
  BspCOMInit.BaudRate   = 115200;
  BspCOMInit.WordLength = COM_WORDLENGTH_8B;
  BspCOMInit.StopBits   = COM_STOPBITS_1;
  BspCOMInit.Parity     = COM_PARITY_NONE;
  BspCOMInit.HwFlowCtl  = COM_HWCONTROL_NONE;
  if (BSP_COM_Init(COM1, &BspCOMInit) != BSP_ERROR_NONE)
  {
    Error_Handler();
  }

  /* USER CODE BEGIN BSP */

  /* -- Sample board code to send message over COM1 port ---- */
  printf("Welcome to STM32 world !\r\n");
  ST7735_Init();
  ST7735_Backlight_On();
  ST7735_SetRotation(3);
//  demoTFT();
  ST7735_FillScreen(ST7735_BLACK);

  /* USER CODE END BSP */

  /* Infinite loop */
  /* USER CODE BEGIN WHILE */
  const char * message = "testing...";
  const uint16_t CENTER_X = ST7735_GetWidth() / 2;
  const uint16_t CENTER_Y = ST7735_GetHeight() / 2;
  uint8_t uart_rx[1] = {0};
  char receive[100] = {0};
  uint8_t receive_index = 0;
  while (1)
  {

    /* -- Sample board code for User push-button in interrupt mode ---- */
    if (BspButtonState == BUTTON_PRESSED)
    {
      /* Update button state */
      BspButtonState = BUTTON_RELEASED;

      /* ..... Perform your action ..... */
      HAL_UART_Transmit(&hlpuart1, (uint8_t *)message, 11, 1000);

    }

    if( HAL_UART_Receive(&hlpuart1, uart_rx, 1, 100) == HAL_OK ) {
    	if(uart_rx[0] == 0) {
    		for( uint8_t i = receive_index; i < 10; i++ ) {
    			receive[ i ] = ' ';
    		}
    		receive[ 10 ] = 0;

    		uint16_t x = CENTER_X - ((16/2) * receive_index);
    		uint16_t y = CENTER_Y - ((26/2) * 1);
    		ST7735_DrawString(x, y, receive,Font_16x26, ST7735_WHITE, ST7735_BLACK);

    		receive_index = 0;
    		memset(receive, 0, 100);
    	} else if( uart_rx[0] >= 48 && uart_rx[0] <= 57 ) {
        	receive[receive_index++] = (char)uart_rx[0];
    	} else {
    		// Invalid characters
    	}
    }
    /* USER CODE END WHILE */

    /* USER CODE BEGIN 3 */
  }
  /* USER CODE END 3 */
}

/**
  * @brief System Clock Configuration
  * @retval None
  */
void SystemClock_Config(void)
{
  RCC_OscInitTypeDef RCC_OscInitStruct = {0};
  RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};

  /** Configure the main internal regulator output voltage
  */
  __HAL_PWR_VOLTAGESCALING_CONFIG(PWR_REGULATOR_VOLTAGE_SCALE3);

  while(!__HAL_PWR_GET_FLAG(PWR_FLAG_VOSRDY)) {}

  /** Initializes the RCC Oscillators according to the specified parameters
  * in the RCC_OscInitTypeDef structure.
  */
  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSI|RCC_OSCILLATORTYPE_CSI;
  RCC_OscInitStruct.HSIState = RCC_HSI_ON;
  RCC_OscInitStruct.HSIDiv = RCC_HSI_DIV1;
  RCC_OscInitStruct.HSICalibrationValue = RCC_HSICALIBRATION_DEFAULT;
  RCC_OscInitStruct.CSIState = RCC_CSI_ON;
  RCC_OscInitStruct.CSICalibrationValue = RCC_CSICALIBRATION_DEFAULT;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
  RCC_OscInitStruct.PLL.PLLSource = RCC_PLL1_SOURCE_CSI;
  RCC_OscInitStruct.PLL.PLLM = 1;
  RCC_OscInitStruct.PLL.PLLN = 32;
  RCC_OscInitStruct.PLL.PLLP = 2;
  RCC_OscInitStruct.PLL.PLLQ = 2;
  RCC_OscInitStruct.PLL.PLLR = 2;
  RCC_OscInitStruct.PLL.PLLRGE = RCC_PLL1_VCIRANGE_2;
  RCC_OscInitStruct.PLL.PLLVCOSEL = RCC_PLL1_VCORANGE_WIDE;
  RCC_OscInitStruct.PLL.PLLFRACN = 0;
  if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK)
  {
    Error_Handler();
  }

  /** Initializes the CPU, AHB and APB buses clocks
  */
  RCC_ClkInitStruct.ClockType = RCC_CLOCKTYPE_HCLK|RCC_CLOCKTYPE_SYSCLK
                              |RCC_CLOCKTYPE_PCLK1|RCC_CLOCKTYPE_PCLK2
                              |RCC_CLOCKTYPE_PCLK3;
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_HSI;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV1;
  RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV1;
  RCC_ClkInitStruct.APB3CLKDivider = RCC_HCLK_DIV1;

  if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_3) != HAL_OK)
  {
    Error_Handler();
  }

  /** Configure the programming delay
  */
  __HAL_FLASH_SET_PROGRAM_DELAY(FLASH_PROGRAMMING_DELAY_1);
}

/**
  * @brief LPUART1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_LPUART1_UART_Init(void)
{

  /* USER CODE BEGIN LPUART1_Init 0 */

  /* USER CODE END LPUART1_Init 0 */

  /* USER CODE BEGIN LPUART1_Init 1 */

  /* USER CODE END LPUART1_Init 1 */
  hlpuart1.Instance = LPUART1;
  hlpuart1.Init.BaudRate = 115200;
  hlpuart1.Init.WordLength = UART_WORDLENGTH_8B;
  hlpuart1.Init.StopBits = UART_STOPBITS_1;
  hlpuart1.Init.Parity = UART_PARITY_NONE;
  hlpuart1.Init.Mode = UART_MODE_TX_RX;
  hlpuart1.Init.HwFlowCtl = UART_HWCONTROL_NONE;
  hlpuart1.Init.OneBitSampling = UART_ONE_BIT_SAMPLE_DISABLE;
  hlpuart1.AdvancedInit.AdvFeatureInit = UART_ADVFEATURE_NO_INIT;
  hlpuart1.FifoMode = UART_FIFOMODE_ENABLE;
  if (HAL_UART_Init(&hlpuart1) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_UARTEx_SetTxFifoThreshold(&hlpuart1, UART_TXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_UARTEx_SetRxFifoThreshold(&hlpuart1, UART_RXFIFO_THRESHOLD_1_8) != HAL_OK)
  {
    Error_Handler();
  }
  if (HAL_UARTEx_EnableFifoMode(&hlpuart1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN LPUART1_Init 2 */

  /* USER CODE END LPUART1_Init 2 */

}

/**
  * @brief SPI1 Initialization Function
  * @param None
  * @retval None
  */
static void MX_SPI1_Init(void)
{

  /* USER CODE BEGIN SPI1_Init 0 */

  /* USER CODE END SPI1_Init 0 */

  /* USER CODE BEGIN SPI1_Init 1 */

  /* USER CODE END SPI1_Init 1 */
  /* SPI1 parameter configuration*/
  hspi1.Instance = SPI1;
  hspi1.Init.Mode = SPI_MODE_MASTER;
  hspi1.Init.Direction = SPI_DIRECTION_2LINES_TXONLY;
  hspi1.Init.DataSize = SPI_DATASIZE_8BIT;
  hspi1.Init.CLKPolarity = SPI_POLARITY_LOW;
  hspi1.Init.CLKPhase = SPI_PHASE_1EDGE;
  hspi1.Init.NSS = SPI_NSS_SOFT;
  hspi1.Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_8;
  hspi1.Init.FirstBit = SPI_FIRSTBIT_MSB;
  hspi1.Init.TIMode = SPI_TIMODE_DISABLE;
  hspi1.Init.CRCCalculation = SPI_CRCCALCULATION_DISABLE;
  hspi1.Init.CRCPolynomial = 0x7;
  hspi1.Init.NSSPMode = SPI_NSS_PULSE_ENABLE;
  hspi1.Init.NSSPolarity = SPI_NSS_POLARITY_LOW;
  hspi1.Init.FifoThreshold = SPI_FIFO_THRESHOLD_01DATA;
  hspi1.Init.MasterSSIdleness = SPI_MASTER_SS_IDLENESS_00CYCLE;
  hspi1.Init.MasterInterDataIdleness = SPI_MASTER_INTERDATA_IDLENESS_00CYCLE;
  hspi1.Init.MasterReceiverAutoSusp = SPI_MASTER_RX_AUTOSUSP_DISABLE;
  hspi1.Init.MasterKeepIOState = SPI_MASTER_KEEP_IO_STATE_DISABLE;
  hspi1.Init.IOSwap = SPI_IO_SWAP_DISABLE;
  hspi1.Init.ReadyMasterManagement = SPI_RDY_MASTER_MANAGEMENT_INTERNALLY;
  hspi1.Init.ReadyPolarity = SPI_RDY_POLARITY_HIGH;
  if (HAL_SPI_Init(&hspi1) != HAL_OK)
  {
    Error_Handler();
  }
  /* USER CODE BEGIN SPI1_Init 2 */

  /* USER CODE END SPI1_Init 2 */

}

/**
  * @brief GPIO Initialization Function
  * @param None
  * @retval None
  */
static void MX_GPIO_Init(void)
{
  GPIO_InitTypeDef GPIO_InitStruct = {0};
/* USER CODE BEGIN MX_GPIO_Init_1 */
/* USER CODE END MX_GPIO_Init_1 */

  /* GPIO Ports Clock Enable */
  __HAL_RCC_GPIOC_CLK_ENABLE();
  __HAL_RCC_GPIOA_CLK_ENABLE();

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(ST7735_DC_GPIO_Port, ST7735_DC_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pin Output Level */
  HAL_GPIO_WritePin(GPIOC, ST7735_RES_Pin|ST7735_BL_Pin|ST7735_CS_Pin, GPIO_PIN_RESET);

  /*Configure GPIO pin : ST7735_DC_Pin */
  GPIO_InitStruct.Pin = ST7735_DC_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(ST7735_DC_GPIO_Port, &GPIO_InitStruct);

  /*Configure GPIO pins : ST7735_RES_Pin ST7735_BL_Pin ST7735_CS_Pin */
  GPIO_InitStruct.Pin = ST7735_RES_Pin|ST7735_BL_Pin|ST7735_CS_Pin;
  GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
  HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);

/* USER CODE BEGIN MX_GPIO_Init_2 */
/* USER CODE END MX_GPIO_Init_2 */
}

/* USER CODE BEGIN 4 */

/* USER CODE END 4 */

/**
  * @brief BSP Push Button callback
  * @param Button Specifies the pressed button
  * @retval None
  */
void BSP_PB_Callback(Button_TypeDef Button)
{
  if (Button == BUTTON_USER)
  {
    BspButtonState = BUTTON_PRESSED;
  }
}

/**
  * @brief  This function is executed in case of error occurrence.
  * @retval None
  */
void Error_Handler(void)
{
  /* USER CODE BEGIN Error_Handler_Debug */
  /* User can add his own implementation to report the HAL error return state */
  __disable_irq();
  while (1)
  {
  }
  /* USER CODE END Error_Handler_Debug */
}

#ifdef  USE_FULL_ASSERT
/**
  * @brief  Reports the name of the source file and the source line number
  *         where the assert_param error has occurred.
  * @param  file: pointer to the source file name
  * @param  line: assert_param error line source number
  * @retval None
  */
void assert_failed(uint8_t *file, uint32_t line)
{
  /* USER CODE BEGIN 6 */
  /* User can add his own implementation to report the file name and line number,
     ex: printf("Wrong parameters value: file %s on line %d\r\n", file, line) */
  /* USER CODE END 6 */
}
#endif /* USE_FULL_ASSERT */
