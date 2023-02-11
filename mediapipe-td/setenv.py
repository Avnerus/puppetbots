import sys
import os
import platform

def onStart():
	user = 'avnerus' # Update accordingly

	condaEnv = 'mediapipe-td' # Update accordingly

	if platform.system() == 'Windows':
		if sys.version_info.major <= 3 and sys.version_info.minor >= 8:
			"""
			Double check all the following paths, it could be that your anaconda 'envs' folder is not in your User folder depending on your conda install settings and conda version.
			"""
			os.add_dll_directory('C:/Users/'+user+'/.conda/envs/'+condaEnv+'/DLLs')
			os.add_dll_directory('C:/Users/'+user+'/.conda/envs/'+condaEnv+'/Library/bin')
		else:
			"""
			Double check all the following paths, it could be that your anaconda 'envs' folder is not in your User folder depending on your conda install settings and conda version.
			"""
			# Not the most elegant solution, but we need to control load order
			os.environ['PATH'] = 'C:/Users/'+user+'/.conda/envs/'+condaEnv+'/DLLs' + os.pathsep + os.environ['PATH']
			os.environ['PATH'] = 'C:/Users/'+user+'/.conda/envs/'+condaEnv+'/Library/bin' + os.pathsep + os.environ['PATH']

		sys.path = ['C:/Users/'+user+'/.conda/envs/'+condaEnv+'/Lib/site-packages'] + sys.path

	else:
		"""
		MacOS users should include path to .dlybs / MacOS binaries, site-packages
		"""
		os.environ['PATH'] = '/Users/'+user+'/opt/anaconda3/envs/'+condaEnv+'/lib' + os.pathsep + os.environ['PATH']
		os.environ['PATH'] = '/Users/'+user+'/opt/anaconda3/envs/'+condaEnv+'/bin' + os.pathsep + os.environ['PATH']
		sys.path = ['/Users/'+user+'/opt/anaconda3/envs/'+condaEnv+'/lib/python3.9/site-packages'] + sys.path

	return