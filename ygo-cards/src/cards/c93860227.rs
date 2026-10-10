//! Necroquip Princess
use super::prelude::*;

pub const ID: u32 = 93860227;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 monster equipped with a Monster Card + 1 Fiend Monster Card
    // Fusion.AddProcMix(c,true,true,s.eqmatfilter,s.fiendmatfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| eqmatfilter(d, c))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| fiendmatfilter(d, c, fc, st, tp))),
        ],
    );
    // Must be Special Summoned by sending the materials to the GY
    // Fusion.AddContactProc(c,function(tp) return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil) end,function(g) Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL) end,true)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contact_group)),
            material_operation: Some(std::sync::Arc::new(contact_op)),
            sumcon: Some(None),
            ..Default::default()
        },
    );
    // You can only control 1 "Necroquip Princess"
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.effcon)
    effect::set_condition(d, e1, Some(condition(effcon)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.eqmatfilter(c)`
fn eqmatfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetEquipGroup():IsExists(Card.IsMonsterCard,1,nil)
    let eqg = card::get_equip_group(d, c);
    group::is_exists(d, &eqg, &|d, x| card::is_monster_card(d, x), 1, NO_EX)
}
/// `s.fiendmatfilter(c,fc,sumtype,tp)`
fn fiendmatfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsMonsterCard() and c:IsRace(RACE_FIEND,fc,sumtype,tp)
    card::is_monster_card(d, c) && card::is_race_ctx(d, c, RACE_FIEND, Some(fc), sumtype, Some(tp))
}
/// `function(tp) return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD|LOCATION_HAND,0,nil) end`
fn contact_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, NO_EX)
}
/// `function(g) Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL) end`
fn contact_op(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.effconfilter(c)`
fn effconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsReason(REASON_COST) and c:IsPreviousLocation(LOCATION_HAND)
    card::is_monster(d, c) && card::is_reason(d, c, REASON_COST) && card::is_previous_location(d, c, LOCATION_HAND)
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.effconfilter,1,nil) and re:IsActivated()
    let eg = a.eg.clone().nil();
    let re = a.re.nil();
    group::is_exists(d, &eg, &|d, c| effconfilter(d, c), 1, NO_EX) && effect::is_activated(d, re)
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return s.effconfilter(c) and c:CheckUniqueOnField(tp) and (c:IsControler(tp) or c:IsAbleToChangeControler())
    //     and not c:IsForbidden() and c:IsLocation(LOCATION_GRAVE)
    effconfilter(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
        && !card::is_forbidden(d, c)
        && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local b1=Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and eg:IsExists(s.eqfilter,1,nil,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && group::is_exists(d, &eg, &|d, c| eqfilter(d, c, tp), 1, NO_EX);
    // local b2=Duel.IsPlayerCanDraw(tp,1)
    let b2 = duel::is_player_can_draw(d, tp, 1);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    let op = op.map_or(0, |x| x as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // Duel.SetTargetCard(eg)
        duel::set_target_card(d, eg.clone());
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.SetOperationInfo(0,CATEGORY_EQUIP,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(eg), 1, tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
        if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.GetTargetCards(e):FilterSelect(tp,s.eqfilter,1,1,nil,tp):GetFirst()
        let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
        let sel = group::filter_select(d, &tg, tp, &|d, x| eqfilter(d, x, tp), 1, 1, false, NO_EX).nil();
        let ec = group::get_first(d, &sel);
        // if ec and Duel.Equip(tp,ec,c) then
        if let Some(ec) = ec {
            if duel::equip(d, tp, ec, c, true, false) {
                // The equipped monster gains 500 ATK
                // local e1=Effect.CreateEffect(ec)
                let e1 = effect::create_effect(d, ec);
                // e1:SetType(EFFECT_TYPE_EQUIP)
                effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(500)
                effect::set_value(d, e1, 500);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // ec:RegisterEffect(e1)
                card::register_effect(d, ec, e1, false);
                // Equip limit
                // local e2=Effect.CreateEffect(ec)
                let e2 = effect::create_effect(d, ec);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e2:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
                // e2:SetValue(function(e,cc) return cc==c end)
                effect::set_value_fn(d, e2, lua_fn(move |_d, x: Args| x.card(1) == Some(c)));
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // ec:RegisterEffect(e2)
                card::register_effect(d, ec, e2, false);
            }
        }
    } else if op == 2 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
