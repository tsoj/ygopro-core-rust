//! Swickelavee the Brilliantly Crowned Heavy Cavalry
//! 燦冠乗騎シックラヴィー
use super::prelude::*;

pub const ID: u32 = 15185344;
/// `local COUNTER_CROWN=0x21c`
const COUNTER_CROWN: u32 = 0x21c;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], counter_place_list: &[COUNTER_CROWN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:EnableCounterPermit(COUNTER_CROWN)
    card::enable_counter_permit(d, c, COUNTER_CROWN as u16, None, None);
    // Xyz Summon procedure: 2 Level 3 monsters, OR, once per turn, you can also Xyz Summon "Swickelavee the Brilliantly Crowned Heavy Cavalry" by using 1 Beast monster you control with 2000 or less ATK
    // Xyz.AddProcedure(c,nil,3,2,s.altxyzmat,aux.Stringid(id,0),2,s.altxyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(3),
        2,
        Some(xyz::alter_filter(|d, cc, tp, xyz| altxyzmat(d, cc, tp, xyz))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        Some(lua_fn(altxyzop)),
        false,
        None,
    );
    // Cannot be used as material for an Xyz Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card destroys a monster by battle: Place 1 Crown Counter on it, then apply this effect based on the number of Crown Counters on it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_COUNTER+CATEGORY_ATKCHANGE+CATEGORY_TOEXTRA+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_COUNTER | CATEGORY_ATKCHANGE | CATEGORY_TOEXTRA | CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(function(e) return e:GetHandler():IsRelateToBattle() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_relate_to_battle(d, h)
        })),
    );
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.altxyzmat(c,tp,xyz)`
fn altxyzmat(d: &mut Duel, c: CardId, tp: u8, xyz: CardId) -> bool {
    // return c:IsRace(RACE_BEAST,xyz,SUMMON_TYPE_XYZ,tp) and c:IsAttackBelow(2000) and c:IsFaceup()
    card::is_race_ctx(d, c, RACE_BEAST, Some(xyz), SUMMON_TYPE_XYZ as u64, Some(tp)) && card::is_attack_below(d, c, 2000) && card::is_faceup(d, c)
}
/// `s.altxyzop(e,tp,chk)`
fn altxyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local crown_counters=c:GetCounter(COUNTER_CROWN)
    let crown_counters = card::get_counter(d, c, COUNTER_CROWN as u16);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,c,1,tp,COUNTER_CROWN)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(c)), 1, tp, COUNTER_CROWN as i32);
    // if crown_counters<3 then
    if crown_counters < 3 {
        // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,c,1,tp,crown_counters==1 and 400 or 600)
        let v = if crown_counters == 1 { 400 } else { 600 };
        duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, tp, v);
    }
    // if crown_counters==2 then
    if crown_counters == 2 {
        // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,1,tp,3)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 3);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:AddCounter(COUNTER_CROWN,1) then
    if card::is_relate_to_effect(d, c, e) && card::add_counter(d, c, COUNTER_CROWN as u16, 1, false) {
        // local crown_counters=c:GetCounter(COUNTER_CROWN)
        let crown_counters = card::get_counter(d, c, COUNTER_CROWN as u16);
        // if crown_counters==1 then
        if crown_counters == 1 {
            // --● 1: This card gains 400 ATK
            // c:UpdateAttack(400)
            card::update_attack(d, c, 400, None, None, None);
        // elseif crown_counters==2 then
        } else if crown_counters == 2 {
            // --● 2: This card gains 600 ATK
            // c:UpdateAttack(600)
            card::update_attack(d, c, 600, None, None, None);
        // elseif crown_counters==3 then
        } else if crown_counters == 3 {
            // --● 3: Return this card to the Extra Deck, and if you do, draw 3 cards
            // if Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and c:IsLocation(LOCATION_EXTRA) then
            if duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 && card::is_location(d, c, LOCATION_EXTRA) {
                // Duel.Draw(tp,3,REASON_EFFECT)
                duel::draw(d, tp, 3, REASON_EFFECT);
            }
        }
    }
}
