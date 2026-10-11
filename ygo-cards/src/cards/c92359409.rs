//! Superdreadnought Rail Cannon Gustav Rocket
//! 超弩級砲塔列車グスタフ・ロケット
use super::prelude::*;

pub const ID: u32 = 92359409;
/// "Superdreadnought Rail Cannon Gustav Max"
const CARD_GUSTAV_MAX: u32 = 56910167;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_GUSTAV_MAX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 10 monsters, OR by discarding 1 card and using "Superdreadnought Rail Cannon Gustav Max" you control
    // Xyz.AddProcedure(c,nil,10,3,s.ovfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(10),
        3,
        Some(xyz::alter_filter(|d, cc, tp, xyzc| ovfilter(d, cc, tp, xyzc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Negate that effect, and if you do, destroy that monster, then inflict 1000 damage to your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DISABLE | CATEGORY_DESTROY | CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 1 material from this card or destroy it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetTarget(s.detachtg)
    effect::set_target(d, e2, Some(cost_target(detachtg)));
    // e2:SetOperation(s.detachop)
    effect::set_operation(d, e2, Some(operation(detachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsSummonCode(xyzc,SUMMON_TYPE_XYZ,tp,56910167) and c:IsFaceup()
    card::is_summon_code(d, c, Some(xyzc), SUMMON_TYPE_XYZ as u64, tp, &[CARD_GUSTAV_MAX]) && card::is_faceup(d, c)
}
/// `s.xyzop(e,tp,chk,mc)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local sc=Duel.GetMatchingGroup(Card.IsDiscardable,tp,LOCATION_HAND,0,nil):SelectUnselect(Group.CreateGroup(),tp,false,Xyz.ProcCancellable)
    let mg = duel::get_matching_group(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, NO_EX);
    let cancellable = xyz::proc_cancellable(d);
    let sc = group::select_unselect(d, &mg, Some(&CardSet::new()), tp, false, cancellable, None, None);
    // if sc and Duel.SendtoGrave(sc,REASON_DISCARD|REASON_COST)>0 then
    if let Some(sc) = sc {
        if duel::send_to_grave(d, sc, REASON_DISCARD | REASON_COST, None, None) > 0 {
            // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
            return true;
        }
    }
    // else return false end
    false
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return rp==1-tp and re:IsMonsterEffect() and e:GetHandler():GetOverlayCount()>0 and Duel.IsChainDisablable(ev)
    if a.rp != 1 - tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::get_overlay_count(d, h) == 0 {
        return false;
    }
    duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,1000,REASON_EFFECT)
            duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
        }
    }
}
/// `s.detachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn detachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetOverlayCount()==0 then
    if card::get_overlay_count(d, c) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.detachop(e,tp,eg,ep,ev,re,r,rp)`
fn detachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local b1=c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    let b1 = card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    // local b2=true
    let b2 = true;
    // --Detach 1 material from this card or destroy it
    // local op=b1 and Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)},
    //     {b2,aux.Stringid(id,4)}) or 2
    let op = if b1 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).unwrap_or(2)
    } else {
        2
    };
    // if op==1 then
    if op == 1 {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT);
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
