<?xml version="1.0" encoding="UTF-8"?>
<!-- Generated with glade 3.22.2 -->
<interface>
  <requires lib="gtk+" version="3.20"/>
  <!-- interface-css-provider-path game.css -->
  <object class="GtkApplicationWindow">
    <property name="can_focus">False</property>
    <child type="titlebar">
      <placeholder/>
    </child>
    <child>
      <object class="GtkBox">
        <property name="visible">True</property>
        <property name="can_focus">False</property>
        <property name="orientation">vertical</property>
        <child>
          <object class="GtkBox">
            <property name="visible">True</property>
            <property name="can_focus">False</property>
            <property name="orientation">vertical</property>
            <child>
              <object class="GtkBox">
                <property name="visible">True</property>
                <property name="can_focus">False</property>
                <child>
                  <object class="GtkDrawingArea">
                    <property name="visible">True</property>
                    <property name="can_focus">False</property>
                  </object>
                  <packing>
                    <property name="expand">False</property>
                    <property name="fill">True</property>
                    <property name="position">0</property>
                  </packing>
                </child>
                <child>
                  <object class="GtkBox">
                    <property name="height_request">50</property>
                    <property name="visible">True</property>
                    <property name="can_focus">False</property>
                    <property name="halign">start</property>
                    <property name="margin_left">10</property>
                    <property name="margin_right">10</property>
                    <property name="margin_top">10</property>
                    <property name="margin_bottom">10</property>
                    <property name="orientation">vertical</property>
                    <child>
                      <object class="GtkLabel">
                        <property name="visible">True</property>
                        <property name="can_focus">False</property>
                        <property name="label" translatable="yes">Текущий профиль:</property>
                        <style>
                          <class name="text"/>
                        </style>
                      </object>
                      <packing>
                        <property name="expand">False</property>
                        <property name="fill">True</property>
                        <property name="position">0</property>
                      </packing>
                    </child>
                    <child>
                      <object class="GtkLabel">
                        <property name="visible">True</property>
                        <property name="can_focus">False</property>
                        <property name="label" translatable="yes">Новый профиль</property>
                        <style>
                          <class name="text"/>
                        </style>
                      </object>
                      <packing>
                        <property name="expand">False</property>
                        <property name="fill">True</property>
                        <property name="position">1</property>
                      </packing>
                    </child>
                  </object>
                  <packing>
                    <property name="expand">False</property>
                    <property name="fill">True</property>
                    <property name="position">1</property>
                  </packing>
                </child>
              </object>
              <packing>
                <property name="expand">False</property>
                <property name="fill">True</property>
                <property name="position">0</property>
              </packing>
            </child>
            <style>
              <class name="header"/>
            </style>
          </object>
          <packing>
            <property name="expand">False</property>
            <property name="fill">True</property>
            <property name="position">0</property>
          </packing>
        </child>
        <child>
          <object class="GtkButtonBox">
            <property name="height_request">300</property>
            <property name="visible">True</property>
            <property name="can_focus">False</property>
            <property name="halign">center</property>
            <property name="valign">center</property>
            <property name="orientation">vertical</property>
            <property name="spacing">20</property>
            <property name="layout_style">spread</property>
            <child>
              <object class="GtkButton">
                <property name="label" translatable="yes">Играть</property>
                <property name="width_request">200</property>
                <property name="visible">True</property>
                <property name="can_focus">False</property>
                <property name="receives_default">True</property>
                <property name="halign">center</property>
              </object>
              <packing>
                <property name="expand">True</property>
                <property name="fill">True</property>
                <property name="position">0</property>
              </packing>
            </child>
            <child>
              <object class="GtkButton">
                <property name="label" translatable="yes">Настройки</property>
                <property name="width_request">200</property>
                <property name="visible">True</property>
                <property name="can_focus">False</property>
                <property name="receives_default">True</property>
                <property name="halign">center</property>
              </object>
              <packing>
                <property name="expand">True</property>
                <property name="fill">True</property>
                <property name="position">1</property>
              </packing>
            </child>
            <child>
              <object class="GtkButton">
                <property name="label" translatable="yes">Выход</property>
                <property name="width_request">200</property>
                <property name="visible">True</property>
                <property name="can_focus">False</property>
                <property name="receives_default">True</property>
                <property name="halign">center</property>
              </object>
              <packing>
                <property name="expand">True</property>
                <property name="fill">True</property>
                <property name="position">2</property>
              </packing>
            </child>
          </object>
          <packing>
            <property name="expand">True</property>
            <property name="fill">True</property>
            <property name="position">1</property>
          </packing>
        </child>
      </object>
    </child>
    <style>
      <class name="main_menu"/>
    </style>
  </object>
</interface>
