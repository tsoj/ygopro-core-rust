//! Performage Trapeze Witch
//! ＥＭトラピーズ・フォーズ・ウィッチ
use super::prelude::*;

pub const ID: u32 = 33206889;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_PERFORMAGE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusiom Materials: 2 "Performage" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PERFORMAGE),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_PERFORMAGE as u16], Some(fc), st, Some(tp)))), 2)],
    );
    // Your "Performage" monsters cannot be targted by opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_PERFORMAGE))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, x| card::is_set_card(d, x, &[SET_PERFORMAGE as u16]))));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your "Performage" monsters cannot be destroyed by your card effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetProperty(0)
    effect::set_property(d, e2, 0, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(aux.indsval)
    effect::set_value_fn(d, e2, aux::indsval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent's monsters cannot target this card for attacks while you control another "Performage" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.cannotatkcon)
    effect::set_condition(d, e3, Some(condition(cannotatkcon)));
    // e3:SetValue(aux.imval2)
    effect::set_value_fn(d, e3, aux::imval2());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Decrease the ATK of an opponent's monster by 600
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cannotatkfilter(c)`
fn cannotatkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PERFORMAGE) and c:IsFaceup() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PERFORMAGE as u16]) && card::is_faceup(d, c) && !card::is_code(d, c, &[ID])
}
/// `s.cannotatkcon(e)`
fn cannotatkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cannotatkfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&cannotatkfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc0,bc1=Duel.GetBattleMonster(tp)
    let (bc0, bc1) = duel::get_battle_monster(d, tp);
    // if chk==0 then return bc0 and bc1 and bc0:IsFaceup() and bc1:IsFaceup() and bc0:IsSetCard(SET_PERFORMAGE) end
    if a.chk == Some(0) {
        return match (bc0, bc1) {
            (Some(bc0), Some(bc1)) => {
                card::is_faceup(d, bc0) && card::is_faceup(d, bc1) && card::is_set_card(d, bc0, &[SET_PERFORMAGE as u16])
            }
            _ => false,
        };
    }
    // e:SetLabelObject(bc1)
    effect::set_label_object(d, a.e, Value::card(bc1));
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,bc1,1,tp,-600)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, bc1.map(CardSet::from_card), 1, tp, -600);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(1-tp) then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - tp) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-600)
            effect::set_value(d, e1, -600);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
