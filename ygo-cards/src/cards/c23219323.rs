//! Junk Warrior/Assault Mode
use super::prelude::*;

pub const ID: u32 = 23219323;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ASSAULT_MODE, CARD_JUNK_WARRIOR],
    listed_series: &[SET_ASSAULT_MODE],
    assault_mode: Some(CARD_JUNK_WARRIOR),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return 1000*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_ASSAULT_MODE),c:GetControler(),LOCATION_MZONE,0,nil) end)
    effect::set_value_fn(d, e1, effect_card_value(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetCondition(function(e) return e:GetHandler():GetBattleTarget() end)
    effect::set_condition(d, e2, Some(lua_fn(e2_condition)));
    // e2:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e2, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() and te:IsMonsterEffect() end)
    effect::set_value_fn(d, e3, lua_fn(immune_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetTarget(s.texsptg)
    effect::set_target(d, e4, Some(cost_target(texsptg)));
    // e4:SetOperation(s.texspop)
    effect::set_operation(d, e4, Some(operation(texspop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function(e,c) return 1000*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_ASSAULT_MODE),c:GetControler(),LOCATION_MZONE,0,nil) end
fn atk_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    let f = aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_ASSAULT_MODE as u16]));
    1000 * duel::get_matching_group_count(d, Some(&f), p, LOCATION_MZONE, 0, NO_EX) as i64
}
// function(e) return e:GetHandler():GetBattleTarget() end
fn e2_condition(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_battle_target(d, c).is_some()
}
// function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() and te:IsMonsterEffect() end
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_activated(d, te) && effect::is_monster_effect(d, te)
}
// function s.texfilter(c)
fn texfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_JUNK_WARRIOR) and c:IsAbleToExtra()
    card::is_code(d, c, &[CARD_JUNK_WARRIOR]) && card::is_able_to_extra(d, c)
}
// function s.texsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn texsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.texfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && texfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.texfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&texfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.texfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&texfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, g, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.texspop(e,tp,eg,ep,ev,re,r,rp)
fn texspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,0,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_deck(d, tc, None, 0, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_EXTRA)
    {
        // local forcedg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,tc,nil,REASON_SYNCHRO)
        let forcedg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(tc), None, REASON_SYNCHRO);
        // if #forcedg==0 and Duel.GetLocationCountFromEx(tp,tp,nil,tc)>0
        //     and tc:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if forcedg.is_empty()
            && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(tc), None).0 > 0
            && card::is_can_be_special_summoned(d, tc, a.e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // tc:SetMaterial(nil)
            card::set_material(d, tc, CardSet::new());
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SpecialSummon(tc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, tc, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // tc:CompleteProcedure()
                card::complete_procedure(d, tc);
            }
        }
    }
}
