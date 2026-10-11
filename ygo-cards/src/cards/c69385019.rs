//! Mermail King - Neptabyss
use super::prelude::*;

pub const ID: u32 = 69385019;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ABYSS, SET_ATLANTEAN, SET_MERMAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a Fish, Sea Serpent, or Aqua monster
    // Link.AddProcedure(c,nil,2,3,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(matcheck)), None, None);
    // Your opponent cannot target WATER monsters this card points to with card effects
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
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(function(e,c) return e:GetHandler():GetLinkedGroup():IsContains(c) and c:IsAttribute(ATTRIBUTE_WATER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        let lg = card::get_linked_group(d, h);
        group::is_contains(&lg, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)
    })));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to your hand, or equip to this card, 1 "Abyss-" Equip Spell from your Deck or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND+CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND + CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Atlantean" or "Mermail" monster from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and e:GetHandler():IsPreviousControler(tp) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        a.rp == 1 - a.tp && card::is_previous_controler(d, h, a.tp)
    })));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matcheck(g,lc,sumtype,tp)
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsRace,1,nil,RACE_FISH|RACE_SEASERPENT|RACE_AQUA,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_race_ctx(d, c, (RACE_FISH | RACE_SEASERPENT | RACE_AQUA) as u64, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// function s.eqconfilter(c)
fn eqconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_COST) and c:IsAttribute(ATTRIBUTE_WATER) and (not c:IsPreviousLocation(LOCATION_MZONE) or c:IsPreviousAttributeOnField(ATTRIBUTE_WATER))
    card::is_reason(d, c, REASON_COST)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && (!card::is_previous_location(d, c, LOCATION_MZONE) || card::is_previous_attribute_on_field(d, c, ATTRIBUTE_WATER))
}
// function s.eqcon(e,tp,eg,ep,ev,re,r,rp)
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsActivated() and eg:IsExists(s.eqconfilter,1,nil)
    let Some(re) = a.re else {
        return false;
    };
    if !effect::is_activated(d, re) {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &eqconfilter, 1, NO_EX)
}
// function s.eqfilter(c,tp,stzone_check,ec)
fn eqfilter(d: &mut Duel, c: CardId, tp: u8, stzone_check: bool, ec: Option<CardId>) -> bool {
    // return c:IsSetCard(SET_ABYSS) and c:IsEquipSpell() and (c:IsAbleToHand()
    //     or (stzone_check and ec and c:CheckEquipTarget(ec) and c:CheckUniqueOnField(tp) and not c:IsForbidden()))
    card::is_set_card(d, c, &[SET_ABYSS as u16])
        && card::is_equip_spell(d, c)
        && (card::is_able_to_hand(d, c, None)
            || (stzone_check
                && ec.is_some_and(|ec| {
                    card::check_equip_target(d, c, ec) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
                })))
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp,Duel.GetLocationCount(tp,LOCATION_SZONE)>0,e:GetHandler()) end
    if a.chk == Some(0) {
        let stzone_check = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
        let h = effect::get_handler(d, a.e);
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| eqfilter(d, c, tp, stzone_check, h)),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then c=nil end
    let c = match effect::get_handler(d, a.e) {
        Some(cc) if card::is_relate_to_effect(d, cc, a.e) && card::is_faceup(d, cc) => Some(cc),
        _ => None,
    };
    // local stzone_check=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let stzone_check = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp,stzone_check,c):GetFirst()
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(move |d: &mut Duel, x| eqfilter(d, x, tp, stzone_check, c))),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // aux.ToHandOrElse(sc,tp,
        //     function()
        //         return stzone_check and c and sc:CheckEquipTarget(c) and sc:CheckUniqueOnField(tp) and not sc:IsForbidden()
        //     end,
        //     function()
        //         Duel.Equip(tp,sc,c)
        //     end,
        //     aux.Stringid(id,3)
        // )
        let check = |d: &mut Duel, _x: CardId| -> bool {
            stzone_check
                && c.is_some_and(|c| card::check_equip_target(d, sc, c) && card::check_unique_on_field(d, sc, tp, None, None) && !card::is_forbidden(d, sc))
        };
        let oper = |d: &mut Duel, _g: &CardSet| -> i32 {
            duel::equip(d, tp, sc, c.nil(), true, false);
            0
        };
        aux::to_hand_or_else(d, sc, tp, Some(&check), Some(&oper), Some(aux::stringid(ID, 3)));
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_ATLANTEAN,SET_MERMAIL}) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ATLANTEAN as u16, SET_MERMAIL as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
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
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
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
