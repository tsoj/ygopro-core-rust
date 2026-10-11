//! Number 101: Silent Honor ARK - Soul Asylum
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 2419596;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(101), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // You can detach 1 material from this card, then activate 1 of these effects;
    // ● Choose 1 Level 4 monster in your GY, except the detached material, and add 1 Level 4 monster with the same Type and Attribute, but a different name, from your Deck to your hand
    // ● Target 1 monster your opponent controls; attach it to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card would be destroyed, you can detach 1 material from this card instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,race,attr,code)`
fn thfilter(d: &mut Duel, c: CardId, race: u64, attr: u32, code: u32) -> bool {
    // return c:IsLevel(4) and c:IsRace(race) and c:IsAttribute(attr) and not c:IsCode(code)
    card::is_level(d, c, &[4]) && card::is_race(d, c, race) && card::is_attribute(d, c, attr) && !card::is_code(d, c, &[code])
}
/// `s.gyfilter(c,tp)`
fn gyfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevel(4) and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetRace(),c:GetAttribute(),c:GetCode())
    if !card::is_level(d, c, &[4]) {
        return false;
    }
    let race = card::get_race(d, c);
    let attr = card::get_attribute(d, c);
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, race, attr, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `local cd=e:GetChainData()` field `choice`: `cd.choice`
fn chain_choice(d: &Duel, e: EffectId) -> i64 {
    chain::effect_data(d, e, None, chain::field::CARD).int()
}
/// `cd.cost_detached_materials`
fn detached_materials(d: &Duel, e: EffectId) -> CardSet {
    chain::effect_data(d, e, None, chain::field::COST_DETACHED_MATERIALS).as_group().cloned().nil()
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cd=Chain.GetCurrentLink()>0 and e:GetChainData() or nil
    let cd = duel::get_current_chain(d, false) > 0;
    // if chkc then return cd and cd.choice==2 and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return cd
            && chain_choice(d, a.e) == 2
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_can_be_xyz_material(d, chkc, Some(c), Some(tp), Some(REASON_EFFECT));
    }
    // local exc=cd and cd.cost_detached_materials:GetFirst() or nil
    let exc = if cd { group::get_first(d, &detached_materials(d, a.e)) } else { None };
    // local option_1=Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_GRAVE,0,1,exc,tp)
    let option_1 = match exc {
        Some(x) => duel::is_existing_matching_card(d, Some(&|d, c| gyfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, x.into()),
        None => duel::is_existing_matching_card(d, Some(&|d, c| gyfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX),
    };
    // local option_2=c:IsXyzMonster() and Duel.IsExistingTarget(Card.IsCanBeXyzMaterial,tp,0,LOCATION_MZONE,1,nil,c,tp,REASON_EFFECT)
    let option_2 = card::is_xyz_monster(d, c)
        && duel::is_existing_target(
            d,
            Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(tp), Some(REASON_EFFECT))),
            tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    // if chk==0 then return option_1 or option_2 end
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    // cd.choice=Duel.SelectEffect(tp,{option_1,aux.Stringid(id,1)},{option_2,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 1)), (option_2, aux::stringid(ID, 2))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice.map_or(Value::Nil, |s| Value::Int(s as i64)));
    match choice {
        // if cd.choice==1 then
        Some(1) => {
            // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
            effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
            // e:SetProperty(0)
            effect::set_property(d, a.e, 0, 0);
            // exc:CreateEffectRelation(e)
            let x = exc.nil();
            card::create_effect_relation(d, x, a.e);
            // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
            duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        }
        // elseif cd.choice==2 then
        Some(2) => {
            // e:SetCategory(0)
            effect::set_category(d, a.e, 0);
            // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
            effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
            // Duel.SelectTarget(tp,Card.IsCanBeXyzMaterial,tp,0,LOCATION_MZONE,1,1,nil,c,tp,REASON_EFFECT)
            let _ = duel::select_target(
                d,
                tp,
                Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(tp), Some(REASON_EFFECT))),
                tp,
                0,
                LOCATION_MZONE,
                1,
                1,
                false,
                NO_EX,
            );
        }
        _ => {}
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local choice=cd.choice
    let choice = chain_choice(d, a.e);
    if choice == 1 {
        // local detached_mat=cd.cost_detached_materials:GetFirst()
        let dm = group::get_first(d, &detached_materials(d, a.e)).nil();
        // local exc=detached_mat:IsRelateToEffect(e) and detached_mat or nil
        let exc = if card::is_relate_to_effect(d, dm, a.e) { Some(dm) } else { None };
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local gyc=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_GRAVE,0,1,1,exc,tp):GetFirst()
        let sel = match exc {
            Some(x) => duel::select_matching_card(d, tp, Some(&|d, c| gyfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, x.into()),
            None => duel::select_matching_card(d, tp, Some(&|d, c| gyfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX),
        }
        .nil();
        // if not gyc then return end
        let gyc = match group::get_first(d, &sel) {
            Some(gyc) => gyc,
            None => return,
        };
        // Duel.HintSelection(gyc)
        duel::hint_selection(d, gyc, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,gyc:GetRace(),gyc:GetAttribute(),gyc:GetCode())
        let race = card::get_race(d, gyc);
        let attr = card::get_attribute(d, gyc);
        let code = card::get_code(d, gyc)[0];
        let g = duel::select_matching_card(d, tp, Some(&|d, x| thfilter(d, x, race, attr, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    } else if choice == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d);
        // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
        if card::is_relate_to_effect(d, c, a.e) {
            let tc = tc.nil();
            if card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_xyz_material(d, tc, Some(c), Some(tp), Some(REASON_EFFECT)) {
                // Duel.Overlay(c,tc,true)
                duel::overlay(d, c, tc, true);
            }
        }
    }
}
/// `s.e2_target` (`function(e,tp,eg,ep,ev,re,r,rp,chk)`): the EFFECT_DESTROY_REPLACE of the card
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) and not c:IsReason(REASON_REPLACE) end
    if a.chk == Some(0) {
        return card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT) && !card::is_reason(d, c, REASON_REPLACE);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0
    duel::select_effect_yes_no(d, a.tp, c, Some(96)) && card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT) > 0
}
