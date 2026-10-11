//! War Rock Mammud
//! Scripted by XyleN
use super::prelude::*;

pub const ID: u32 = 84903021;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WAR_ROCK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal Summon without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(lua_fn(sumcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 Spell/Trap & "War Rock" gains 200 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsRace(RACE_WARRIOR)
    card::is_facedown(d, c) || !card::is_race(d, c, RACE_WARRIOR)
}
// function s.sumcon(e,c,minc,zone)
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let minc = a.int(2);
    let zone = a.int(3) as u32;
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //     and (Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 or not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil))
    minc == 0
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
        && (duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
            || !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX))
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetBattleMonster(tp)
    // return bc and bc:IsAttribute(ATTRIBUTE_EARTH) and bc:IsRace(RACE_WARRIOR)
    match duel::get_battle_monster(d, a.tp).0 {
        Some(bc) => card::is_attribute(d, bc, ATTRIBUTE_EARTH) && card::is_race(d, bc, RACE_WARRIOR),
        None => false,
    }
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WAR_ROCK) and c:IsFaceup() and not c:IsStatus(STATUS_BATTLE_DESTROYED)
    card::is_set_card(d, c, &[SET_WAR_ROCK as u16]) && card::is_faceup(d, c) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // local atkg=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,0,nil)
        let atkg = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // if #atkg==0 then return end
        if group::get_count(&atkg) == 0 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for tc in atkg:Iter() do
        for tc in group::iter(d, &atkg) {
            // --Increase ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
            // e1:SetValue(200)
            effect::set_value(d, e1, 200);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
