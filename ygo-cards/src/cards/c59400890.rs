//! Dark Magician of Destruction
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 59400890;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_DARK_MAGICIAN],
    material_setcode: &[SET_DARK_MAGICIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Dark Magician" + 1 LIGHT or DARK monster
    // Fusion.AddProcMix(c,true,true,CARD_DARK_MAGICIAN,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT|ATTRIBUTE_DARK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_DARK_MAGICIAN),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK, Some(fc), st, Some(tp))
            })),
        ],
    );
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // You can only Fusion Summon or Special Summon by its alternate procedure "Dark Magician of Destruction" once per turn
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can Special Summon this card by banishing 1 Level 6 or higher DARK Spellcaster monster you control during the turn a Spell Card or effect was activated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e1, LOCATION_EXTRA);
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(lua_fn(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Dark Magician" or 1 card that mentions it from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // This card's name becomes "Dark Magician" while on the field or in the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e3, EFFECT_CHANGE_CODE);
    // e3:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_MZONE | LOCATION_GRAVE);
    // e3:SetValue(CARD_DARK_MAGICIAN)
    effect::set_value(d, e3, CARD_DARK_MAGICIAN as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Keep track of a turn a Spell Card or effect was activated
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,function(re,tp,cid) return not re:IsSpellEffect() end)
    duel::add_custom_activity_counter(
        d,
        ID,
        ACTIVITY_CHAIN,
        lua_fn(|d, a: Args| {
            let re = a.e(0);
            !effect::is_spell_effect(d, re)
        }),
    );
}
/// s.regcon(e)
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() or c:IsSummonType(SUMMON_TYPE_SPECIAL+1)
    card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
}
/// s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Prevent another Fusion Summon or Special Summon by its alternate procedure of "Dark Magician of Destruction" that turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return c:IsOriginalCode(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1) end)
    effect::set_target(
        d,
        e1,
        Some(lua_fn(|d, a: Args| {
            let c = a.c(1);
            let sumtype = a.int(3) as u64;
            card::is_original_code(d, c, &[ID])
                && ((sumtype & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
                    || (sumtype & (SUMMON_TYPE_SPECIAL as u64 + 1)) == SUMMON_TYPE_SPECIAL as u64 + 1)
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// s.selfspcostfilter(c,tp,sc)
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, sc: Option<CardId>) -> bool {
    // return c:IsLevelAbove(6) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_SPELLCASTER)
    //     and c:IsAbleToRemoveAsCost() and Duel.GetLocationCountFromEx(tp,tp,c,sc)>0
    card::is_level_above(d, c, 6)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_able_to_remove_as_cost(d, c, None)
        && {
            let sc_card = match sc {
                Some(s) => duel::ExtraCard::Card(s),
                None => duel::ExtraCard::None,
            };
            duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), sc_card, None).0 > 0
        }
}
/// s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return (Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)>0 or Duel.GetCustomActivityCount(id,1-tp,ACTIVITY_CHAIN)>0)
    //     and Duel.IsExistingMatchingCard(s.selfspcostfilter,tp,LOCATION_MZONE,0,1,nil,tp,c)
    (duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) > 0 || duel::get_custom_activity_count(d, ID, 1 - tp, ACTIVITY_CHAIN) > 0)
        && duel::is_existing_matching_card(d, Some(&|d, x| selfspcostfilter(d, x, tp, Some(c))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.card(9);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.selfspcostfilter,tp,LOCATION_MZONE,0,1,1,true,nil,tp,c)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| selfspcostfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, 1, true, NO_EX);
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // g:KeepAlive()
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            // return true
            return true;
        }
    }
    // return false
    false
}
/// s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
    // g:DeleteGroup()
}
/// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_DARK_MAGICIAN) or c:ListsCode(CARD_DARK_MAGICIAN)) and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_DARK_MAGICIAN]) || card::lists_code(d, c, &[CARD_DARK_MAGICIAN])) && card::is_able_to_hand(d, c, None)
}
/// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
