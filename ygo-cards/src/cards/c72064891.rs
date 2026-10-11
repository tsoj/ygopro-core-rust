//! Curse of Dragon, the Magical Knight Dragon
//! Scripted by edo9300
use super::prelude::*;

pub const ID: u32 = 72064891;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GAIA_CHAMPION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_WARRIOR as u64, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| matfilter(d, x, fc, st, tp))),
        ],
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_EXTRA_FUSION_MATERIAL);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_GRAVE,0)
    effect::set_target_range(d, e1, LOCATION_GRAVE, 0);
    // e1:SetTarget(function(e,c) return c:IsAbleToRemove() and c:IsMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_able_to_remove(d, c, None, Some(POS_FACEUP as u8), Some(REASON_EFFECT)) && card::is_monster(d, c))));
    // e1:SetOperation(Fusion.BanishMaterial)
    effect::set_operation(d, e1, Some(fusion::banish_material()));
    // e1:SetValue(function(e,c) return c and c:IsRace(RACE_DRAGON) and c:IsControler(e:GetHandlerPlayer()) and c:IsLevel(7) end)
    effect::set_value_fn(
        d,
        e1,
        lua_fn(|d, a| {
            let Some(c) = a.card(1) else {
                return false;
            };
            let hp = effect::get_handler_player(d, a.e(0));
            card::is_race(d, c, RACE_DRAGON as u64) && card::is_controler(d, c, hp) && card::is_level(d, c, &[7])
        }),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 Spell/Trap that mentions "Gaia the Dragon Champion" from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::is_fusion_summoned(d, c)
        })),
    );
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,fc,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsLevelAbove(5)
    card::is_race_ctx(d, c, RACE_DRAGON as u64, Some(fc), sumtype, Some(tp)) && card::is_level_above(d, c, 5)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_GAIA_CHAMPION) and c:IsSpellTrap() and c:IsAbleToHand()
    card::lists_code(d, c, &[CARD_GAIA_CHAMPION]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
